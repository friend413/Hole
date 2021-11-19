package p

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strings"
	"time"

	"net/http"

	"github.com/certusone/wormhole/node/pkg/vaa"
)

const cgBaseUrl = "https://api.coingecko.com/api/v3/"
const cgProBaseUrl = "https://pro-api.coingecko.com/api/v3/"

type CoinGeckoCoin struct {
	Id     string `json:"id"`
	Symbol string `json:"symbol"`
	Name   string `json:"name"`
}
type CoinGeckoCoins []CoinGeckoCoin

type CoinGeckoMarket [2]float64

type CoinGeckoMarketRes struct {
	Prices []CoinGeckoMarket `json:"prices"`
}
type CoinGeckoErrorRes struct {
	Error string `json:"error"`
}

func fetchCoinGeckoCoins() map[string][]CoinGeckoCoin {
	baseUrl := cgBaseUrl
	cgApiKey := os.Getenv("COINGECKO_API_KEY")
	if cgApiKey != "" {
		baseUrl = cgProBaseUrl
	}
	url := fmt.Sprintf("%vcoins/list", baseUrl)
	req, reqErr := http.NewRequest("GET", url, nil)
	if reqErr != nil {
		log.Fatalf("failed coins request, err: %v", reqErr)
	}

	if cgApiKey != "" {
		req.Header.Set("X-Cg-Pro-Api-Key", cgApiKey)
	}

	res, resErr := http.DefaultClient.Do(req)
	if resErr != nil {
		log.Fatalf("failed get coins response, err: %v", resErr)
	}

	defer res.Body.Close()
	body, bodyErr := ioutil.ReadAll(res.Body)
	if bodyErr != nil {
		log.Fatalf("failed decoding coins body, err: %v", bodyErr)
	}

	var parsed []CoinGeckoCoin

	parseErr := json.Unmarshal(body, &parsed)
	if parseErr != nil {
		log.Printf("failed parsing body. err %v\n", parseErr)
	}
	var geckoCoins = map[string][]CoinGeckoCoin{}
	for _, coin := range parsed {
		symbol := strings.ToLower(coin.Symbol)
		geckoCoins[symbol] = append(geckoCoins[symbol], coin)
	}
	return geckoCoins

}

func chainIdToCoinGeckoPlatform(chain vaa.ChainID) string {
	switch chain {
	case vaa.ChainIDSolana:
		return "solana"
	case vaa.ChainIDEthereum:
		return "ethereum"
	case vaa.ChainIDTerra:
		return "terra"
	case vaa.ChainIDBSC:
		return "binance-smart-chain"
	case vaa.ChainIDPolygon:
		return "polygon-pos"
	}
	return ""
}

func fetchCoinGeckoCoinFromContract(chainId vaa.ChainID, address string) CoinGeckoCoin {
	baseUrl := cgBaseUrl
	cgApiKey := os.Getenv("COINGECKO_API_KEY")
	if cgApiKey != "" {
		baseUrl = cgProBaseUrl
	}
	platform := chainIdToCoinGeckoPlatform(chainId)
	url := fmt.Sprintf("%vcoins/%v/contract/%v", baseUrl, platform, address)
	req, reqErr := http.NewRequest("GET", url, nil)
	if reqErr != nil {
		log.Fatalf("failed contract request, err: %v\n", reqErr)
	}
	if cgApiKey != "" {
		req.Header.Set("X-Cg-Pro-Api-Key", cgApiKey)
	}

	res, resErr := http.DefaultClient.Do(req)
	if resErr != nil {
		log.Fatalf("failed get contract response, err: %v\n", resErr)
	}

	defer res.Body.Close()
	body, bodyErr := ioutil.ReadAll(res.Body)
	if bodyErr != nil {
		log.Fatalf("failed decoding contract body, err: %v\n", bodyErr)
	}

	var parsed CoinGeckoCoin

	parseErr := json.Unmarshal(body, &parsed)
	if parseErr != nil {
		log.Printf("failed parsing body. err %v\n", parseErr)
		var errRes CoinGeckoErrorRes
		if err := json.Unmarshal(body, &errRes); err == nil {
			if errRes.Error == "Could not find coin with the given id" {
				log.Printf("Could not find CoinGecko coin by contract address, for chain %v, address, %v\n", chainId, address)
			} else {
				log.Println("Failed calling CoinGecko, got err", errRes.Error)
			}
		}
	}

	return parsed
}

func fetchCoinGeckoCoinId(chainId vaa.ChainID, address, symbol, name string) (coinId, foundSymbol, foundName string) {
	// try coingecko, return if good
	// if coingecko does not work, try chain-specific options

	// initialize strings that will be returned if we find a symbol/name
	// when looking up this token by contract address
	newSymbol := ""
	newName := ""

	if symbol == "" && chainId == vaa.ChainIDSolana {
		// try to lookup the symbol in solana token list, from the address
		if token, ok := solanaTokens[address]; ok {
			symbol = token.Symbol
			name = token.Name
			newSymbol = token.Symbol
			newName = token.Name
		}
	}
	if _, ok := coinGeckoCoins[strings.ToLower(symbol)]; ok {
		tokens := coinGeckoCoins[strings.ToLower(symbol)]
		if len(tokens) == 1 {
			// only one match found for this symbol
			return tokens[0].Id, newSymbol, newName
		}
		for _, token := range tokens {
			if token.Name == name {
				// found token by name match
				return token.Id, newSymbol, newName
			}
			if strings.Contains(strings.ToLower(strings.ReplaceAll(name, " ", "")), strings.ReplaceAll(token.Id, "-", "")) {
				// found token by id match
				log.Println("found token by symbol and name match", name)
				return token.Id, newSymbol, newName
			}
		}
		// more than one symbol with this name, let contract lookup try
	}
	coin := fetchCoinGeckoCoinFromContract(chainId, address)
	if coin.Id != "" {
		return coin.Id, newSymbol, newName
	}
	// could not find a CoinGecko coin
	return "", newSymbol, newName
}

func fetchCoinGeckoPrice(coinId string, timestamp time.Time) (float64, error) {
	hourAgo := time.Now().Add(-time.Duration(1) * time.Hour)
	withinLastHour := timestamp.After(hourAgo)
	start, end := rangeFromTime(timestamp, 4)

	baseUrl := cgBaseUrl
	cgApiKey := os.Getenv("COINGECKO_API_KEY")
	if cgApiKey != "" {
		baseUrl = cgProBaseUrl
	}
	url := fmt.Sprintf("%vcoins/%v/market_chart/range?vs_currency=usd&from=%v&to=%v", baseUrl, coinId, start.Unix(), end.Unix())
	req, reqErr := http.NewRequest("GET", url, nil)
	if reqErr != nil {
		log.Fatalf("failed coins request, err: %v\n", reqErr)
	}
	if cgApiKey != "" {
		req.Header.Set("X-Cg-Pro-Api-Key", cgApiKey)
	}

	res, resErr := http.DefaultClient.Do(req)
	if resErr != nil {
		log.Fatalf("failed get coins response, err: %v\n", resErr)
	}

	defer res.Body.Close()
	body, bodyErr := ioutil.ReadAll(res.Body)
	if bodyErr != nil {
		log.Fatalf("failed decoding coins body, err: %v\n", bodyErr)
	}

	var parsed CoinGeckoMarketRes

	parseErr := json.Unmarshal(body, &parsed)
	if parseErr != nil {
		log.Printf("failed parsing body. err %v\n", parseErr)
		var errRes CoinGeckoErrorRes
		if err := json.Unmarshal(body, &errRes); err == nil {
			log.Println("Failed calling CoinGecko, got err", errRes.Error)
		}
	}
	if len(parsed.Prices) >= 1 {
		var priceIndex int
		if withinLastHour {
			// use the last price in the list, latest price
			priceIndex = len(parsed.Prices) - 1
		} else {
			// use a price from the middle of the list, as that should be
			// closest to the timestamp.
			numPrices := len(parsed.Prices)
			priceIndex = numPrices / 2
		}
		price := parsed.Prices[priceIndex][1]
		fmt.Printf("found a price for %v! %v\n", coinId, price)
		return price, nil
	}
	fmt.Println("no price found in coinGecko for", coinId)
	return 0, fmt.Errorf("no price found for %v", coinId)
}

const solanaTokenListURL = "https://raw.githubusercontent.com/solana-labs/token-list/main/src/tokens/solana.tokenlist.json"

type SolanaToken struct {
	Address  string `json:"address"`
	Symbol   string `json:"symbol"`
	Name     string `json:"name"`
	Decimals int    `json:"decimals"`
}
type SolanaTokenListRes struct {
	Tokens []SolanaToken `json:"tokens"`
}

func fetchSolanaTokenList() map[string]SolanaToken {

	req, reqErr := http.NewRequest("GET", solanaTokenListURL, nil)
	if reqErr != nil {
		log.Fatalf("failed solana token list request, err: %v", reqErr)
	}

	res, resErr := http.DefaultClient.Do(req)
	if resErr != nil {
		log.Fatalf("failed get solana token list response, err: %v", resErr)
	}

	defer res.Body.Close()
	body, bodyErr := ioutil.ReadAll(res.Body)
	if bodyErr != nil {
		log.Fatalf("failed decoding solana token list body, err: %v", bodyErr)
	}

	var parsed SolanaTokenListRes

	parseErr := json.Unmarshal(body, &parsed)
	if parseErr != nil {
		log.Printf("failed parsing body. err %v\n", parseErr)
	}
	var solTokens = map[string]SolanaToken{}
	for _, token := range parsed.Tokens {
		if _, ok := solTokens[token.Address]; !ok {
			solTokens[token.Address] = token
		}
	}
	return solTokens
}
