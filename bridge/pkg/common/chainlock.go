package common

import (
	"github.com/certusone/wormhole/bridge/pkg/vaa"
	"time"

	"github.com/ethereum/go-ethereum/common"
)

type MessagePublication struct {
	TxHash    common.Hash // TODO: rename to identifier? on Solana, this isn't actually the tx hash
	Timestamp time.Time

	Nonce          uint32
	EmitterChain   vaa.ChainID
	EmitterAddress vaa.Address
	Payload        []byte
}
