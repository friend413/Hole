package processor

import (
	"context"
	"encoding/hex"

	"github.com/ethereum/go-ethereum/crypto"
	"go.uber.org/zap"

	"github.com/certusone/wormhole/bridge/pkg/supervisor"
	"github.com/certusone/wormhole/bridge/pkg/vaa"
)

// handleInjection processes a pre-populated VAA injected locally.
func (p *Processor) handleInjection(ctx context.Context, v *vaa.VAA) {
	// Check if we're in the guardian set.
	us, ok := p.gs.KeyIndex(p.ourAddr)
	if !ok {
		p.logger.Error("we're not in the guardian set - refusing to sign",
			zap.Uint32("index", p.gs.Index),
			zap.Stringer("our_addr", p.ourAddr),
			zap.Any("set", p.gs.KeysAsHexStrings()))
		return
	}

	// Generate digest of the unsigned VAA.
	digest, err := v.SigningMsg()
	if err != nil {
		panic(err)
	}

	// The internal originator is responsible for logging the full VAA, just log the digest here.
	supervisor.Logger(ctx).Info("signing injected VAA",
		zap.Stringer("digest", digest))

	// Sign the digest using our node's guardian key.
	s, err := crypto.Sign(digest.Bytes(), p.gk)
	if err != nil {
		panic(err)
	}

	p.logger.Info("observed and signed injected VAA",
		zap.String("digest", hex.EncodeToString(digest.Bytes())),
		zap.String("signature", hex.EncodeToString(s)),
		zap.Int("our_index", us))

	p.broadcastSignature(v, s)
}
