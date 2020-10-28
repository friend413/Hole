package guardiand

import (
	"context"
	"fmt"
	"net/http"
	_ "net/http/pprof"
	"os"
	"syscall"

	eth_common "github.com/ethereum/go-ethereum/common"
	"github.com/libp2p/go-libp2p-core/crypto"
	"github.com/libp2p/go-libp2p-core/peer"
	"github.com/spf13/cobra"
	"go.uber.org/zap"
	"golang.org/x/sys/unix"

	"github.com/certusone/wormhole/bridge/pkg/common"
	"github.com/certusone/wormhole/bridge/pkg/devnet"
	"github.com/certusone/wormhole/bridge/pkg/ethereum"
	"github.com/certusone/wormhole/bridge/pkg/p2p"
	"github.com/certusone/wormhole/bridge/pkg/processor"
	gossipv1 "github.com/certusone/wormhole/bridge/pkg/proto/gossip/v1"
	solana "github.com/certusone/wormhole/bridge/pkg/solana"
	"github.com/certusone/wormhole/bridge/pkg/supervisor"
	"github.com/certusone/wormhole/bridge/pkg/vaa"

	ipfslog "github.com/ipfs/go-log/v2"
)

var (
	p2pNetworkID *string
	p2pPort      *uint
	p2pBootstrap *string

	nodeKeyPath *string

	ethRPC           *string
	ethContract      *string
	ethConfirmations *uint64

	agentRPC *string

	logLevel *string

	unsafeDevMode   *bool
	devNumGuardians *uint
	nodeName        *string
)

func init() {
	p2pNetworkID = BridgeCmd.Flags().String("network", "/wormhole/dev", "P2P network identifier")
	p2pPort = BridgeCmd.Flags().Uint("port", 8999, "P2P UDP listener port")
	p2pBootstrap = BridgeCmd.Flags().String("bootstrap", "", "P2P bootstrap peers (comma-separated)")

	nodeKeyPath = BridgeCmd.Flags().String("nodeKey", "", "Path to node key (will be generated if it doesn't exist)")

	ethRPC = BridgeCmd.Flags().String("ethRPC", "", "Ethereum RPC URL")
	ethContract = BridgeCmd.Flags().String("ethContract", "", "Ethereum bridge contract address")
	ethConfirmations = BridgeCmd.Flags().Uint64("ethConfirmations", 15, "Ethereum confirmation count requirement")

	agentRPC = BridgeCmd.Flags().String("agentRPC", "", "Solana agent sidecar gRPC address")

	logLevel = BridgeCmd.Flags().String("logLevel", "info", "Logging level (debug, info, warn, error, dpanic, panic, fatal)")

	unsafeDevMode = BridgeCmd.Flags().Bool("unsafeDevMode", false, "Launch node in unsafe, deterministic devnet mode")
	devNumGuardians = BridgeCmd.Flags().Uint("devNumGuardians", 5, "Number of devnet guardians to include in guardian set")
	nodeName = BridgeCmd.Flags().String("nodeName", "", "Node name to announce in gossip heartbeats")
}

var (
	rootCtx       context.Context
	rootCtxCancel context.CancelFunc
)

// "Why would anyone do this?" are famous last words.
//
// We already forcibly override RPC URLs and keys in dev mode to prevent security
// risks from operator error, but an extra warning won't hurt.
const devwarning = `
        +++++++++++++++++++++++++++++++++++++++++++++++++++
        |   NODE IS RUNNING IN INSECURE DEVELOPMENT MODE  |
        |                                                 |
        |      Do not use -unsafeDevMode in prod.         |
        +++++++++++++++++++++++++++++++++++++++++++++++++++

`

func rootLoggerName() string {
	if *unsafeDevMode {
		// FIXME: add hostname to root logger for cleaner console output in multi-node development.
		// The proper way is to change the output format to include the hostname.
		hostname, err := os.Hostname()
		if err != nil {
			panic(err)
		}

		return fmt.Sprintf("%s-%s", "wormhole", hostname)
	} else {
		return "wormhole"
	}
}

// BridgeCmd represents the bridge command
var BridgeCmd = &cobra.Command{
	Use:   "bridge",
	Short: "Run the bridge server",
	Run:   runBridge,
}

func runBridge(cmd *cobra.Command, args []string) {
	if *unsafeDevMode {
		fmt.Print(devwarning)
	}

	// Lock current and future pages in memory to protect secret keys from being swapped out to disk.
	// It's possible (and strongly recommended) to deploy Wormhole such that keys are only ever
	// stored in memory and never touch the disk. This is a privileged operation and requires CAP_IPC_LOCK.
	err := unix.Mlockall(syscall.MCL_CURRENT | syscall.MCL_FUTURE)
	if err != nil {
		fmt.Printf("Failed to lock memory: %v (CAP_IPC_LOCK missing?)\n", err)
		os.Exit(1)
	}

	// Set up logging. The go-log zap wrapper that libp2p uses is compatible with our
	// usage of zap in supervisor, which is nice.
	lvl, err := ipfslog.LevelFromString(*logLevel)
	if err != nil {
		fmt.Println("Invalid log level")
		os.Exit(1)
	}

	// Our root logger. Convert directly to a regular Zap logger.
	logger := ipfslog.Logger(rootLoggerName()).Desugar()

	// Override the default go-log config, which uses a magic environment variable.
	ipfslog.SetAllLoggers(lvl)

	// In devnet mode, we automatically set a number of flags that rely on deterministic keys.
	if *unsafeDevMode {
		go func() {
			logger.Info("debug server listening on [::]:6060")
			logger.Error("debug server crashed", zap.Error(http.ListenAndServe("[::]:6060", nil)))
		}()

		g0key, err := peer.IDFromPrivateKey(devnet.DeterministicP2PPrivKeyByIndex(0))
		if err != nil {
			panic(err)
		}

		// Use the first guardian node as bootstrap
		*p2pBootstrap = fmt.Sprintf("/dns4/guardian-0.guardian/udp/%d/quic/p2p/%s", *p2pPort, g0key.String())

		// Deterministic ganache ETH devnet address.
		*ethContract = devnet.BridgeContractAddress.Hex()

		// Use the hostname as nodeName. For production, we don't want to do this to
		// prevent accidentally leaking sensitive hostnames.
		hostname, err := os.Hostname()
		if err != nil {
			panic(err)
		}
		*nodeName = hostname
	}

	// Verify flags

	if *nodeKeyPath == "" && !*unsafeDevMode { // In devnet mode, keys are deterministically generated.
		logger.Fatal("Please specify -nodeKey")
	}
	if *agentRPC == "" {
		logger.Fatal("Please specify -agentRPC")
	}
	if *ethRPC == "" {
		logger.Fatal("Please specify -ethRPC")
	}
	if *ethContract == "" {
		logger.Fatal("Please specify -ethContract")
	}
	if *nodeName == "" {
		logger.Fatal("Please specify -nodeName")
	}

	ethContractAddr := eth_common.HexToAddress(*ethContract)

	// Guardian key
	gk := loadGuardianKey(logger)

	// Node's main lifecycle context.
	rootCtx, rootCtxCancel = context.WithCancel(context.Background())
	defer rootCtxCancel()

	// Ethereum lock event channel
	lockC := make(chan *common.ChainLock)

	// Ethereum incoming guardian set updates
	setC := make(chan *common.GuardianSet)

	// Outbound gossip message queue
	sendC := make(chan []byte)

	// Inbound observations
	obsvC := make(chan *gossipv1.LockupObservation, 50)

	// VAAs to submit to Solana
	solanaVaaC := make(chan *vaa.VAA)

	// Load p2p private key
	var priv crypto.PrivKey
	if *unsafeDevMode {
		idx, err := devnet.GetDevnetIndex()
		if err != nil {
			logger.Fatal("Failed to parse hostname - are we running in devnet?")
		}
		priv = devnet.DeterministicP2PPrivKeyByIndex(int64(idx))
	} else {
		priv, err = getOrCreateNodeKey(logger, *nodeKeyPath)
		if err != nil {
			logger.Fatal("Failed to load node key", zap.Error(err))
		}
	}

	// Run supervisor.
	supervisor.New(rootCtx, logger, func(ctx context.Context) error {
		if err := supervisor.Run(ctx, "p2p", p2p.Run(
			obsvC, sendC, priv, *p2pPort, *p2pNetworkID, *p2pBootstrap, *nodeName, rootCtxCancel)); err != nil {
			return err
		}

		if err := supervisor.Run(ctx, "ethwatch",
			ethereum.NewEthBridgeWatcher(*ethRPC, ethContractAddr, *ethConfirmations, lockC, setC).Run); err != nil {
			return err
		}

		if err := supervisor.Run(ctx, "solwatch",
			solana.NewSolanaBridgeWatcher(*agentRPC, lockC, solanaVaaC).Run); err != nil {
			return err
		}

		p := processor.NewProcessor(ctx, lockC, setC, sendC, obsvC, solanaVaaC, gk, *unsafeDevMode, *devNumGuardians, *ethRPC)
		if err := supervisor.Run(ctx, "processor", p.Run); err != nil {
			return err
		}

		logger.Info("Started internal services")

		select {
		case <-ctx.Done():
			return nil
		}
	},
		// It's safer to crash and restart the process in case we encounter a panic,
		// rather than attempting to reschedule the runnable.
		supervisor.WithPropagatePanic)

	select {
	case <-rootCtx.Done():
		logger.Info("root context cancelled, exiting...")
		// TODO: wait for things to shut down gracefully
	}
}
