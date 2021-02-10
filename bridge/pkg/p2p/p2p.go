package p2p

import (
	"context"
	"fmt"
	"github.com/certusone/wormhole/bridge/pkg/version"
	"github.com/prometheus/client_golang/prometheus"
	"strings"
	"time"

	"github.com/libp2p/go-libp2p-core/peer"
	"github.com/multiformats/go-multiaddr"

	"github.com/libp2p/go-libp2p"
	connmgr "github.com/libp2p/go-libp2p-connmgr"
	"github.com/libp2p/go-libp2p-core/crypto"
	"github.com/libp2p/go-libp2p-core/host"
	"github.com/libp2p/go-libp2p-core/protocol"
	"github.com/libp2p/go-libp2p-core/routing"
	dht "github.com/libp2p/go-libp2p-kad-dht"
	pubsub "github.com/libp2p/go-libp2p-pubsub"
	libp2pquic "github.com/libp2p/go-libp2p-quic-transport"
	libp2ptls "github.com/libp2p/go-libp2p-tls"
	"go.uber.org/zap"
	"google.golang.org/protobuf/proto"

	gossipv1 "github.com/certusone/wormhole/bridge/pkg/proto/gossip/v1"
	"github.com/certusone/wormhole/bridge/pkg/supervisor"
)

var (
	p2pHeartbeatsSent = prometheus.NewCounter(
		prometheus.CounterOpts{
			Name: "wormhole_p2p_heartbeats_sent_total",
			Help: "Total number of p2p heartbeats sent",
		})
	p2pMessagesSent = prometheus.NewCounter(
		prometheus.CounterOpts{
			Name: "wormhole_p2p_broadcast_messages_sent_total",
			Help: "Total number of p2p pubsub broadcast messages sent",
		})
	p2pMessagesReceived = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "wormhole_p2p_broadcast_messages_received_total",
			Help: "Total number of p2p pubsub broadcast messages received",
		}, []string{"type"})
)

func init() {
	prometheus.MustRegister(p2pHeartbeatsSent)
	prometheus.MustRegister(p2pMessagesSent)
	prometheus.MustRegister(p2pMessagesReceived)
}

func Run(obsvC chan *gossipv1.SignedObservation,
	sendC chan []byte,
	priv crypto.PrivKey,
	port uint,
	networkID string,
	bootstrapPeers string,
	nodeName string,
	rootCtxCancel context.CancelFunc) func(ctx context.Context) error {

	return func(ctx context.Context) (re error) {
		logger := supervisor.Logger(ctx)

		h, err := libp2p.New(ctx,
			// Use the keypair we generated
			libp2p.Identity(priv),

			// Multiple listen addresses
			libp2p.ListenAddrStrings(
				// Listen on QUIC only.
				// https://github.com/libp2p/go-libp2p/issues/688
				fmt.Sprintf("/ip4/0.0.0.0/udp/%d/quic", port),
				fmt.Sprintf("/ip6/::/udp/%d/quic", port),
			),

			// Enable TLS security as the only security protocol.
			libp2p.Security(libp2ptls.ID, libp2ptls.New),

			// Enable QUIC transport as the only transport.
			libp2p.Transport(libp2pquic.NewTransport),

			// Let's prevent our peer from having too many
			// connections by attaching a connection manager.
			libp2p.ConnectionManager(connmgr.NewConnManager(
				100,         // Lowwater
				400,         // HighWater,
				time.Minute, // GracePeriod
			)),

			// Let this host use the DHT to find other hosts
			libp2p.Routing(func(h host.Host) (routing.PeerRouting, error) {
				// TODO(leo): Persistent data store (i.e. address book)
				idht, err := dht.New(ctx, h, dht.Mode(dht.ModeServer),
					// TODO(leo): This intentionally makes us incompatible with the global IPFS DHT
					dht.ProtocolPrefix(protocol.ID("/"+networkID)),
				)
				return idht, err
			}),
		)

		if err != nil {
			panic(err)
		}

		defer func() {
			// TODO: libp2p cannot be cleanly restarted (https://github.com/libp2p/go-libp2p/issues/992)
			logger.Error("p2p routine has exited, cancelling root context...", zap.Error(re))
			rootCtxCancel()
		}()

		logger.Info("Connecting to bootstrap peers", zap.String("bootstrap_peers", bootstrapPeers))

		topic := fmt.Sprintf("%s/%s", networkID, "broadcast")

		logger.Info("Subscribing pubsub topic", zap.String("topic", topic))
		ps, err := pubsub.NewGossipSub(ctx, h)
		if err != nil {
			panic(err)
		}

		th, err := ps.Join(topic)
		if err != nil {
			return fmt.Errorf("failed to join topic: %w", err)
		}

		sub, err := th.Subscribe()
		if err != nil {
			return fmt.Errorf("failed to subscribe topic: %w", err)
		}

		// Add our own bootstrap nodes

		// Count number of successful connection attempts. If we fail to connect to any bootstrap peer, kill
		// the service and have supervisor retry it.
		successes := 0
		// Are we a bootstrap node? If so, it's okay to not have any peers.
		bootstrapNode := false

		for _, addr := range strings.Split(bootstrapPeers, ",") {
			if addr == "" {
				continue
			}
			ma, err := multiaddr.NewMultiaddr(addr)
			if err != nil {
				logger.Error("Invalid bootstrap address", zap.String("peer", addr), zap.Error(err))
				continue
			}
			pi, err := peer.AddrInfoFromP2pAddr(ma)
			if err != nil {
				logger.Error("Invalid bootstrap address", zap.String("peer", addr), zap.Error(err))
				continue
			}

			if pi.ID == h.ID() {
				logger.Info("We're a bootstrap node")
				bootstrapNode = true
				continue
			}

			if err = h.Connect(ctx, *pi); err != nil {
				logger.Error("Failed to connect to bootstrap peer", zap.String("peer", addr), zap.Error(err))
			} else {
				successes += 1
			}
		}

		// TODO: continually reconnect to bootstrap nodes?
		if successes == 0 && !bootstrapNode {
			return fmt.Errorf("failed to connect to any bootstrap peer")
		} else {
			logger.Info("Connected to bootstrap peers", zap.Int("num", successes))
		}

		logger.Info("Node has been started", zap.String("peer_id", h.ID().String()),
			zap.String("addrs", fmt.Sprintf("%v", h.Addrs())))

		go func() {
			ctr := int64(0)
			tick := time.NewTicker(15 * time.Second)
			defer tick.Stop()

			for {
				select {
				case <-ctx.Done():
					return
				case <-tick.C:
					DefaultRegistry.mu.Lock()
					networks := make([]*gossipv1.Heartbeat_Network, 0, len(DefaultRegistry.networkStats))
					for _, v := range DefaultRegistry.networkStats {
						networks = append(networks, v)
					}

					msg := gossipv1.GossipMessage{Message: &gossipv1.GossipMessage_Heartbeat{
						Heartbeat: &gossipv1.Heartbeat{
							NodeName:     nodeName,
							Counter:      ctr,
							Timestamp:    time.Now().UnixNano(),
							Networks:     networks,
							Version:      version.Version(),
							GuardianAddr: DefaultRegistry.guardianAddress,
						}}}

					b, err := proto.Marshal(&msg)
					if err != nil {
						panic(err)
					}
					DefaultRegistry.mu.Unlock()

					err = th.Publish(ctx, b)
					if err != nil {
						logger.Warn("failed to publish heartbeat message", zap.Error(err))
					}

					p2pHeartbeatsSent.Inc()
					ctr += 1
				}
			}
		}()

		go func() {
			for {
				select {
				case <-ctx.Done():
					return
				case msg := <-sendC:
					err := th.Publish(ctx, msg)
					p2pMessagesSent.Inc()
					if err != nil {
						logger.Error("failed to publish message from queue", zap.Error(err))
					}
				}
			}
		}()

		for {
			envelope, err := sub.Next(ctx)
			if err != nil {
				return fmt.Errorf("failed to receive pubsub message: %w", err)
			}

			var msg gossipv1.GossipMessage
			err = proto.Unmarshal(envelope.Data, &msg)
			if err != nil {
				logger.Info("received invalid message",
					zap.String("data", string(envelope.Data)),
					zap.String("from", envelope.GetFrom().String()))
				p2pMessagesReceived.WithLabelValues("invalid").Inc()
				continue
			}

			if envelope.GetFrom() == h.ID() {
				logger.Debug("received message from ourselves, ignoring",
					zap.Any("payload", msg.Message))
				p2pMessagesReceived.WithLabelValues("loopback").Inc()
				continue
			}

			logger.Debug("received message",
				zap.Any("payload", msg.Message),
				zap.Binary("raw", envelope.Data),
				zap.String("from", envelope.GetFrom().String()))

			switch m := msg.Message.(type) {
			case *gossipv1.GossipMessage_Heartbeat:
				logger.Debug("heartbeat received",
					zap.Any("value", m.Heartbeat),
					zap.String("from", envelope.GetFrom().String()))
				p2pMessagesReceived.WithLabelValues("heartbeat").Inc()
			case *gossipv1.GossipMessage_SignedObservation:
				obsvC <- m.SignedObservation
				p2pMessagesReceived.WithLabelValues("observation").Inc()
			default:
				p2pMessagesReceived.WithLabelValues("unknown").Inc()
				logger.Warn("received unknown message type (running outdated software?)",
					zap.Any("payload", msg.Message),
					zap.Binary("raw", envelope.Data),
					zap.String("from", envelope.GetFrom().String()))
			}
		}
	}
}
