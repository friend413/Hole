package ethereum

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	eth_common "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"go.uber.org/zap"

	"github.com/certusone/wormhole/bridge/pkg/common"
	"github.com/certusone/wormhole/bridge/pkg/ethereum/abi"
	"github.com/certusone/wormhole/bridge/pkg/supervisor"
	"github.com/certusone/wormhole/bridge/pkg/vaa"
)

type (
	EthBridgeWatcher struct {
		url              string
		bridge           eth_common.Address
		minConfirmations uint64

		pendingLocks      map[eth_common.Hash]*pendingLock
		pendingLocksGuard sync.Mutex

		evChan chan *common.ChainLock
	}

	pendingLock struct {
		lock *common.ChainLock
		height uint64
	}
)

func NewEthBridgeWatcher(url string, bridge eth_common.Address, minConfirmations uint64, events chan *common.ChainLock) *EthBridgeWatcher {
	return &EthBridgeWatcher{url: url, bridge: bridge, minConfirmations: minConfirmations, evChan: events, pendingLocks: map[eth_common.Hash]*pendingLock{}}
}

func (e *EthBridgeWatcher) Run(ctx context.Context) error {
	c, err := ethclient.DialContext(ctx, e.url)
	if err != nil {
		return fmt.Errorf("dialing eth client failed: %w", err)
	}

	f, err := abi.NewWormholeBridgeFilterer(e.bridge, c)
	if err != nil {
		return fmt.Errorf("could not create wormhole bridge filter: %w", err)
	}

	sink := make(chan *abi.WormholeBridgeLogTokensLocked, 2)
	eventSubscription, err := f.WatchLogTokensLocked(&bind.WatchOpts{
		Context: ctx,
	}, sink, nil, nil)
	if err != nil {
		return fmt.Errorf("failed to subscribe to eth events: %w", err)
	}
	defer eventSubscription.Unsubscribe()

	errC := make(chan error)
	logger := supervisor.Logger(ctx)

	go func() {
		for {
			select {
			case <-ctx.Done():
				return
			case e := <-eventSubscription.Err():
				errC <- e
				return
			case ev := <-sink:
				lock := &common.ChainLock{
					TxHash:        ev.Raw.TxHash,
					SourceAddress: ev.Sender,
					TargetAddress: ev.Recipient,
					SourceChain:   vaa.ChainIDEthereum,
					TargetChain:   vaa.ChainID(ev.TargetChain),
					TokenChain:    vaa.ChainID(ev.TokenChain),
					TokenAddress:  ev.Token,
					Amount:        ev.Amount,
				}

				logger.Info("found new lockup transaction", zap.Stringer("tx", ev.Raw.TxHash),
					zap.Uint64("number", ev.Raw.BlockNumber))
				e.pendingLocksGuard.Lock()
				e.pendingLocks[ev.Raw.TxHash] = &pendingLock{
					lock:   lock,
					height: ev.Raw.BlockNumber,
				}
				e.pendingLocksGuard.Unlock()
			}
		}
	}()

	// Watch headers
	headSink := make(chan *types.Header, 2)
	headerSubscription, err := c.SubscribeNewHead(ctx, headSink)
	if err != nil {
		return fmt.Errorf("failed to subscribe to header events: %w", err)
	}
	defer headerSubscription.Unsubscribe()

	go func() {
		for {
			select {
			case <-ctx.Done():
				return
			case e := <-headerSubscription.Err():
				errC <- e
				return
			case ev := <-headSink:
				start := time.Now()
				logger.Info("processing new header", zap.Stringer("number", ev.Number))
				e.pendingLocksGuard.Lock()

				blockNumberU := ev.Number.Uint64()
				for hash, pLock := range e.pendingLocks {

					// Transaction was dropped and never picked up again
					if pLock.height+4*e.minConfirmations <= blockNumberU {
						logger.Debug("lockup timed out", zap.Stringer("tx", pLock.lock.TxHash),
							zap.Stringer("number", ev.Number))
						delete(e.pendingLocks, hash)
						continue
					}

					// Transaction is now ready
					if pLock.height+e.minConfirmations <= ev.Number.Uint64() {
						logger.Debug("lockup confirmed", zap.Stringer("tx", pLock.lock.TxHash),
							zap.Stringer("number", ev.Number))
						delete(e.pendingLocks, hash)
						e.evChan <- pLock.lock
					}
				}

				e.pendingLocksGuard.Unlock()
				logger.Info("processed new header", zap.Stringer("number", ev.Number),
					zap.Duration("took", time.Since(start)))
			}
		}
	}()

	supervisor.Signal(ctx, supervisor.SignalHealthy)

	select {
	case <-ctx.Done():
		return ctx.Err()
	case err := <-errC:
		return err
	}
}
