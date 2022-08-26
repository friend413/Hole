// This file contains the token config to be used in the mainnet environment.
//
// This file was generated: Tue Aug 16 2022 21:13:27 GMT+0000 (Coordinated Universal Time) using a min notional of 0

package governor

func tokenList() []tokenConfigEntry {
	return []tokenConfigEntry{
		tokenConfigEntry{chain: 1, addr: "37998ccbf2d0458b615cbcc6b1a367c4749e9fef7306622e1b1b58910120bc9a", symbol: "RAY", coinGeckoId: "raydium", decimals: 6, price: 0.848522},                    // Addr: 4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R, Notional: 304346
		tokenConfigEntry{chain: 1, addr: "6271cb7119476b9dce00d815c8ff315fc8bf7d2848633d34942adfd535f2defe", symbol: "stSOL", coinGeckoId: "lido-staked-sol", decimals: 8, price: 45.64},             // Addr: 7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj, Notional: 110552
		tokenConfigEntry{chain: 1, addr: "8c77f3661d6b4a8ef39dbc5340eead8c3cbe0b45099840e8263d8725b587b073", symbol: "ATLAS", coinGeckoId: "star-atlas", decimals: 8, price: 0.00763502},             // Addr: ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx, Notional: 168897
		tokenConfigEntry{chain: 1, addr: "c6fa7af3bedbad3a3d65f36aabc97431b1bbe4c2d2f6e0e47ca60203452f5d61", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                  // Addr: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v, Notional: 6193588
		tokenConfigEntry{chain: 1, addr: "ce010e60afedb22717bd63192f54145a3f965a33bb82d2c7029eb2ce1e208264", symbol: "USDT", coinGeckoId: "tether", decimals: 6, price: 0.999273},                    // Addr: Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB, Notional: 1313358
		tokenConfigEntry{chain: 1, addr: "05718b04572312d73aa71deaec43c89d77844b0b7ff9e3e72da8510182627455", symbol: "BLOCK", coinGeckoId: "blockasset", decimals: 6, price: 0.074435},               // Addr: NFTUkR4u7wKxy9QLaX2TGvd9oZSWoMo4jqSJqdMb7Nk, Notional: 654028
		tokenConfigEntry{chain: 1, addr: "069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f00000000001", symbol: "SOL", coinGeckoId: "wrapped-solana", decimals: 8, price: 43.35},                // Addr: So11111111111111111111111111111111111111112, Notional: 5524868
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000009178997aff09a67d4caccfeb897fb79d036214", symbol: "1SOL", coinGeckoId: "1sol", decimals: 8, price: 0.03722639},                    // Addr: 0x009178997aff09a67d4caccfeb897fb79d036214, Notional: 505465
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000000316eb71485b0ab14103307bf65a021042c6d380", symbol: "HBTC", coinGeckoId: "huobi-btc", decimals: 8, price: 23938},                    // Addr: 0x0316eb71485b0ab14103307bf65a021042c6d380, Notional: 9006
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000005d3606d5c81eb9b7b18530995ec9b29da05faba", symbol: "TOMOE", coinGeckoId: "tomoe", decimals: 8, price: 0.630177},                    // Addr: 0x05d3606d5c81eb9b7b18530995ec9b29da05faba, Notional: 63017
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000008d967bb0134f2d07f7cfb6e246680c53927dd30", symbol: "MATH", coinGeckoId: "math", decimals: 8, price: 0.162438},                      // Addr: 0x08d967bb0134f2d07f7cfb6e246680c53927dd30, Notional: 61197
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000000bc529c00c6401aef6d220be8c6ea1667f6ad93e", symbol: "YFI", coinGeckoId: "yearn-finance", decimals: 8, price: 11085.67},              // Addr: 0x0bc529c00c6401aef6d220be8c6ea1667f6ad93e, Notional: 585954
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000000c572544a4ee47904d54aaa6a970af96b6f00e1b", symbol: "WAS", coinGeckoId: "wasder", decimals: 8, price: 0.02245193},                   // Addr: 0x0c572544a4ee47904d54aaa6a970af96b6f00e1b, Notional: 284342
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000000d8775f648430679a709e98d2b0cb6250d2887ef", symbol: "BAT", coinGeckoId: "basic-attention-token", decimals: 8, price: 0.431009},      // Addr: 0x0d8775f648430679a709e98d2b0cb6250d2887ef, Notional: 69075
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000000f5d2fb29fb7d3cfee444a200298f468908cc942", symbol: "MANA", coinGeckoId: "decentraland", decimals: 8, price: 1.039},                 // Addr: 0x0f5d2fb29fb7d3cfee444a200298f468908cc942, Notional: 209135
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000111111111117dc0aa78b770fa6a738034120c302", symbol: "1INCH", coinGeckoId: "1inch", decimals: 8, price: 0.820716},                    // Addr: 0x111111111117dc0aa78b770fa6a738034120c302, Notional: 139632
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000018aaa7115705e8be94bffebde57af9bfc265b998", symbol: "AUDIO", coinGeckoId: "audius", decimals: 8, price: 0.369141},                   // Addr: 0x18aaa7115705e8be94bffebde57af9bfc265b998, Notional: 3295031
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000001a7e4e63778b4f12a199c062f3efdd288afcbce8", symbol: "agEUR", coinGeckoId: "ageur", decimals: 8, price: 1.039},                       // Addr: 0x1a7e4e63778b4f12a199c062f3efdd288afcbce8, Notional: 230476
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000001f9840a85d5af5bf1d1762f925bdaddc4201f984", symbol: "UNI", coinGeckoId: "uniswap", decimals: 8, price: 8.34},                        // Addr: 0x1f9840a85d5af5bf1d1762f925bdaddc4201f984, Notional: 5236646
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000002260fac5e5542a773aa44fbcfedf7c193bc2c599", symbol: "WBTC", coinGeckoId: "wrapped-bitcoin", decimals: 8, price: 23944},              // Addr: 0x2260fac5e5542a773aa44fbcfedf7c193bc2c599, Notional: 776114
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000027702a26126e0b3702af63ee09ac4d1a084ef628", symbol: "ALEPH", coinGeckoId: "aleph", decimals: 8, price: 0.261478},                    // Addr: 0x27702a26126e0b3702af63ee09ac4d1a084ef628, Notional: 4506550
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000002ba592f78db6436527729929aaf6c908497cb200", symbol: "CREAM", coinGeckoId: "cream-2", decimals: 8, price: 19.88},                     // Addr: 0x2ba592f78db6436527729929aaf6c908497cb200, Notional: 57652
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000002c537e5624e4af88a7ae4060c022609376c8d0eb", symbol: "TRYB", coinGeckoId: "bilira", decimals: 6, price: 0.055262},                    // Addr: 0x2c537e5624e4af88a7ae4060c022609376c8d0eb, Notional: 277042
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000002e95cea14dd384429eb3c4331b776c4cfbb6fcd9", symbol: "THN", coinGeckoId: "throne", decimals: 8, price: 0.00432363},                   // Addr: 0x2e95cea14dd384429eb3c4331b776c4cfbb6fcd9, Notional: 1557766
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000030d20208d987713f46dfd34ef128bb16c404d10f", symbol: "SD", coinGeckoId: "stader", decimals: 8, price: 0.503113},                      // Addr: 0x30d20208d987713f46dfd34ef128bb16c404d10f, Notional: 408302
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000003432b6a60d23ca0dfca7761b7ab56459d9c964d0", symbol: "FXS", coinGeckoId: "frax-share", decimals: 8, price: 5.93},                     // Addr: 0x3432b6a60d23ca0dfca7761b7ab56459d9c964d0, Notional: 624915
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000003845badade8e6dff049820680d1f14bd3903a5d0", symbol: "SAND", coinGeckoId: "the-sandbox", decimals: 8, price: 1.28},                   // Addr: 0x3845badade8e6dff049820680d1f14bd3903a5d0, Notional: 124864
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000045804880de22913dafe09f4980848ece6ecbaf78", symbol: "PAXG", coinGeckoId: "pax-gold", decimals: 8, price: 1771},                      // Addr: 0x45804880de22913dafe09f4980848ece6ecbaf78, Notional: 783383
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000004674672bcddda2ea5300f5207e1158185c944bc0", symbol: "GXT", coinGeckoId: "gem-exchange-and-trading", decimals: 8, price: 0.02879837}, // Addr: 0x4674672bcddda2ea5300f5207e1158185c944bc0, Notional: 536755
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000476c5e26a75bd202a9683ffd34359c0cc15be0ff", symbol: "SRM", coinGeckoId: "serum", decimals: 6, price: 1.032},                         // Addr: 0x476c5e26a75bd202a9683ffd34359c0cc15be0ff, Notional: 3906413
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000004bd70556ae3f8a6ec6c4080a0c327b24325438f3", symbol: "HXRO", coinGeckoId: "hxro", decimals: 8, price: 0.184292},                      // Addr: 0x4bd70556ae3f8a6ec6c4080a0c327b24325438f3, Notional: 2748639
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000004da34f8264cb33a5c9f17081b9ef5ff6091116f4", symbol: "ELFI", coinGeckoId: "elyfi", decimals: 8, price: 0.01414865},                   // Addr: 0x4da34f8264cb33a5c9f17081b9ef5ff6091116f4, Notional: 187898
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000050d1c9771902476076ecfc8b2a83ad6b9355a4c9", symbol: "FTX Token", coinGeckoId: "ftx-token", decimals: 8, price: 30.82},               // Addr: 0x50d1c9771902476076ecfc8b2a83ad6b9355a4c9, Notional: 93772228
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000514910771af9ca656af840dff83e8264ecf986ca", symbol: "LINK", coinGeckoId: "chainlink", decimals: 8, price: 8.52},                     // Addr: 0x514910771af9ca656af840dff83e8264ecf986ca, Notional: 5335684
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000005a98fcbea516cf06857215779fd812ca3bef1b32", symbol: "LDO", coinGeckoId: "lido-dao", decimals: 8, price: 2.59},                       // Addr: 0x5a98fcbea516cf06857215779fd812ca3bef1b32, Notional: 6569505
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000005ab6a4f46ce182356b6fa2661ed8ebcafce995ad", symbol: "SPRT", coinGeckoId: "sportium", decimals: 8, price: 0.351139},                  // Addr: 0x5ab6a4f46ce182356b6fa2661ed8ebcafce995ad, Notional: 12717461
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000065e6b60ea01668634d68d0513fe814679f925bad", symbol: "PIXEL", coinGeckoId: "pixelverse", decimals: 8, price: 0.00106176},             // Addr: 0x65e6b60ea01668634d68d0513fe814679f925bad, Notional: 129532
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000006b175474e89094c44da98b954eedeac495271d0f", symbol: "DAI", coinGeckoId: "dai", decimals: 8, price: 0.998668},                        // Addr: 0x6b175474e89094c44da98b954eedeac495271d0f, Notional: 3554046
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000006b3595068778dd592e39a122f4f5a5cf09c90fe2", symbol: "SUSHI", coinGeckoId: "sushi", decimals: 8, price: 1.43},                        // Addr: 0x6b3595068778dd592e39a122f4f5a5cf09c90fe2, Notional: 5832629
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000727f064a78dc734d33eec18d5370aef32ffd46e4", symbol: "ORION", coinGeckoId: "orion-money", decimals: 8, price: 0.00281795},            // Addr: 0x727f064a78dc734d33eec18d5370aef32ffd46e4, Notional: 134433
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000072b886d09c117654ab7da13a14d603001de0b777", symbol: "XDEFI", coinGeckoId: "xdefi", decimals: 8, price: 0.172598},                    // Addr: 0x72b886d09c117654ab7da13a14d603001de0b777, Notional: 495609
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000007f39c581f595b53c5cb19bd0b3f8da6c935e2ca0", symbol: "wstETH", coinGeckoId: "wrapped-steth", decimals: 8, price: 1980.12},            // Addr: 0x7f39c581f595b53c5cb19bd0b3f8da6c935e2ca0, Notional: 538932
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000007fc66500c84a76ad7e9c93437bfc5ac33e2ddae9", symbol: "AAVE", coinGeckoId: "aave", decimals: 8, price: 108.6},                         // Addr: 0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9, Notional: 121165
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000853d955acef822db058eb8505911ed77f175b99e", symbol: "FRAX", coinGeckoId: "frax", decimals: 8, price: 1},                             // Addr: 0x853d955acef822db058eb8505911ed77f175b99e, Notional: 160537
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000008564653879a18c560e7c0ea0e084c516c62f5653", symbol: "UBXT", coinGeckoId: "upbots", decimals: 8, price: 0.00658272},                  // Addr: 0x8564653879a18c560e7c0ea0e084c516c62f5653, Notional: 80524
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000085eee30c52b0b379b046fb0f85f4f3dc3009afec", symbol: "KEEP", coinGeckoId: "keep-network", decimals: 8, price: 0.197113},              // Addr: 0x85eee30c52b0b379b046fb0f85f4f3dc3009afec, Notional: 93620
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000008a9c67fee641579deba04928c4bc45f66e26343a", symbol: "JRT", coinGeckoId: "jarvis-reward-token", decimals: 8, price: 0.03575436},      // Addr: 0x8a9c67fee641579deba04928c4bc45f66e26343a, Notional: 97938
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000008ce9137d39326ad0cd6491fb5cc0cba0e089b6a9", symbol: "SXP", coinGeckoId: "swipe", decimals: 8, price: 0.483592},                      // Addr: 0x8ce9137d39326ad0cd6491fb5cc0cba0e089b6a9, Notional: 124779
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000092d6c1e31e14520e676a687f0a93788b716beff5", symbol: "DYDX", coinGeckoId: "dydx", decimals: 8, price: 2.03},                          // Addr: 0x92d6c1e31e14520e676a687f0a93788b716beff5, Notional: 247775
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000095ad61b0a150d79219dcf64e1e6cc01f0b64c4ce", symbol: "SHIB", coinGeckoId: "shiba-inu", decimals: 8, price: 0.00001614},               // Addr: 0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce, Notional: 495146
		tokenConfigEntry{chain: 2, addr: "0000000000000000000000009b83f827928abdf18cf1f7e67053572b9bceff3a", symbol: "ARTEM", coinGeckoId: "artem", decimals: 8, price: 0.00894593},                  // Addr: 0x9b83f827928abdf18cf1f7e67053572b9bceff3a, Notional: 98106
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                  // Addr: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48, Notional: 17707469
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000bb0e17ef65f82ab018d8edd776e8dd940327b28b", symbol: "AXS", coinGeckoId: "axie-infinity", decimals: 8, price: 18.11},                 // Addr: 0xbb0e17ef65f82ab018d8edd776e8dd940327b28b, Notional: 56153
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000c00e94cb662c3520282e6f5717214004a7f26888", symbol: "COMP", coinGeckoId: "compound-governance-token", decimals: 8, price: 61.11},    // Addr: 0xc00e94cb662c3520282e6f5717214004a7f26888, Notional: 161502
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", symbol: "WETH", coinGeckoId: "ethereum", decimals: 8, price: 1882.76},                   // Addr: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, Notional: 191414150
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000c944e90c64b2c07662a292be6244bdf05cda44a7", symbol: "GRT", coinGeckoId: "the-graph", decimals: 8, price: 0.131319},                  // Addr: 0xc944e90c64b2c07662a292be6244bdf05cda44a7, Notional: 96998
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000d49efa7bc0d339d74f487959c573d518ba3f8437", symbol: "COLI", coinGeckoId: "shield-finance", decimals: 8, price: 0.00084121},          // Addr: 0xd49efa7bc0d339d74f487959c573d518ba3f8437, Notional: 135847
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7", symbol: "USDT", coinGeckoId: "tether", decimals: 6, price: 0.999273},                    // Addr: 0xdac17f958d2ee523a2206206994597c13d831ec7, Notional: 14014409
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000dfdb7f72c1f195c5951a234e8db9806eb0635346", symbol: "NFD", coinGeckoId: "feisty-doge-nft", decimals: 8, price: 0.00005076},          // Addr: 0xdfdb7f72c1f195c5951a234e8db9806eb0635346, Notional: 137735
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000e28b3b32b6c345a34ff64674606124dd5aceca30", symbol: "INJ", coinGeckoId: "injective-protocol", decimals: 8, price: 1.92},             // Addr: 0xe28b3b32b6c345a34ff64674606124dd5aceca30, Notional: 332797
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000e831f96a7a1dce1aa2eb760b1e296c6a74caa9d5", symbol: "NEXM", coinGeckoId: "nexum", decimals: 8, price: 0.264265},                     // Addr: 0xe831f96a7a1dce1aa2eb760b1e296c6a74caa9d5, Notional: 53383918
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000ef19f4e48830093ce5bc8b3ff7f903a0ae3e9fa1", symbol: "BOTX", coinGeckoId: "botxcoin", decimals: 8, price: 0.0313902},                 // Addr: 0xef19f4e48830093ce5bc8b3ff7f903a0ae3e9fa1, Notional: 67529
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000f17e65822b568b3903685a7c9f496cf7656cc6c2", symbol: "BICO", coinGeckoId: "biconomy", decimals: 8, price: 0.593688},                  // Addr: 0xf17e65822b568b3903685a7c9f496cf7656cc6c2, Notional: 1159943
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000f1f955016ecbcd7321c7266bccfb96c68ea5e49b", symbol: "RLY", coinGeckoId: "rally-2", decimals: 8, price: 0.0417475},                   // Addr: 0xf1f955016ecbcd7321c7266bccfb96c68ea5e49b, Notional: 4774851
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000f8c3527cc04340b208c854e985240c02f7b7793f", symbol: "FRONT", coinGeckoId: "frontier-token", decimals: 8, price: 0.275458},           // Addr: 0xf8c3527cc04340b208c854e985240c02f7b7793f, Notional: 446100
		tokenConfigEntry{chain: 2, addr: "000000000000000000000000fd09911130e6930bf87f2b0554c44f400bd80d3e", symbol: "ETHIX", coinGeckoId: "ethichub", decimals: 8, price: 0.255996},                 // Addr: 0xfd09911130e6930bf87f2b0554c44f400bd80d3e, Notional: 1633516
		tokenConfigEntry{chain: 2, addr: "00000000000000000000000085f17cf997934a597031b2e18a9ab6ebd4b9f6a4", symbol: "NEAR", coinGeckoId: "near", decimals: 8, price: 4.330},                         // *** manually added. Near on ethereum
		tokenConfigEntry{chain: 3, addr: "0000000000000000000000008f5cd460d57ac54e111646fc569179144c7f0c28", symbol: "PLY", coinGeckoId: "playnity", decimals: 6, price: 0.00955359},                 // Addr: terra13awdgcx40tz5uygkgm79dytez3x87rpg4uhnvu, Notional: 1073506
		tokenConfigEntry{chain: 3, addr: "0000000000000000000000002c71557d2edfedd8330e52be500058a014d329e7", symbol: "BTL", coinGeckoId: "bitlocus", decimals: 6, price: 0.00214964},                 // Addr: terra193c42lfwmlkasvcw22l9qqzc5q2dx208tkd7wl, Notional: 1058903
		tokenConfigEntry{chain: 3, addr: "000000000000000000000000b8ae5604d7858eaa46197b19494b595b586e466c", symbol: "aUST", coinGeckoId: "anchorust", decimals: 6, price: 0.04135693},               // Addr: terra1hzh9vpxhsk8253se0vv5jj6etdvxu3nv8z07zu, Notional: 156051
		tokenConfigEntry{chain: 3, addr: "010000000000000000000000000000000000000000000000000000756c756e61", symbol: "LUNA", coinGeckoId: "terra-luna", decimals: 6, price: 0.00009726},              // Addr: uluna, Notional: 13419315
		tokenConfigEntry{chain: 3, addr: "0100000000000000000000000000000000000000000000000000000075757364", symbol: "UST", coinGeckoId: "terrausd", decimals: 6, price: 0.02607664},                 // Addr: uusd, Notional: 6044853
		tokenConfigEntry{chain: 4, addr: "0000000000000000000000002170ed0880ac9a755fd29b2688956bd959f933f8", symbol: "ETH", coinGeckoId: "weth", decimals: 8, price: 1881.9},                         // Addr: 0x2170ed0880ac9a755fd29b2688956bd959f933f8, Notional: 55300
		tokenConfigEntry{chain: 4, addr: "0000000000000000000000003019bf2a2ef8040c242c9a4c5c4bd4c81678b2a1", symbol: "GMT", coinGeckoId: "stepn", decimals: 8, price: 1.09},                          // Addr: 0x3019bf2a2ef8040c242c9a4c5c4bd4c81678b2a1, Notional: 544022
		tokenConfigEntry{chain: 4, addr: "00000000000000000000000055d398326f99059ff775485246999027b3197955", symbol: "USDT", coinGeckoId: "tether", decimals: 8, price: 0.999273},                    // Addr: 0x55d398326f99059ff775485246999027b3197955, Notional: 3629272
		tokenConfigEntry{chain: 4, addr: "0000000000000000000000007e46d5eb5b7ca573b367275fee94af1945f5b636", symbol: "ABST", coinGeckoId: "abitshadow-token", decimals: 8, price: 0.00001902},        // Addr: 0x7e46d5eb5b7ca573b367275fee94af1945f5b636, Notional: 217570
		tokenConfigEntry{chain: 4, addr: "0000000000000000000000008ac76a51cc950d9822d68b83fe1ad97b32cd580d", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 8, price: 0.999041},                  // Addr: 0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d, Notional: 233907
		tokenConfigEntry{chain: 4, addr: "000000000000000000000000bb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c", symbol: "WBNB", coinGeckoId: "wbnb", decimals: 8, price: 317.28},                        // Addr: 0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c, Notional: 8953792
		tokenConfigEntry{chain: 4, addr: "000000000000000000000000e9e7cea3dedca5984780bafc599bd69add087d56", symbol: "BUSD", coinGeckoId: "binance-usd", decimals: 8, price: 0.999999},               // Addr: 0xe9e7cea3dedca5984780bafc599bd69add087d56, Notional: 5932859
		tokenConfigEntry{chain: 4, addr: "000000000000000000000000fa40d8fc324bcdd6bbae0e086de886c571c225d4", symbol: "WZRD", coinGeckoId: "wizardia", decimals: 8, price: 0.02011742},                // Addr: 0xfa40d8fc324bcdd6bbae0e086de886c571c225d4, Notional: 16275
		tokenConfigEntry{chain: 4, addr: "000000000000000000000000fafd4cb703b25cb22f43d017e7e0d75febc26743", symbol: "WEYU", coinGeckoId: "weyu", decimals: 8, price: 0.00175709},                    // Addr: 0xfafd4cb703b25cb22f43d017e7e0d75febc26743, Notional: 7288349
		tokenConfigEntry{chain: 5, addr: "0000000000000000000000000d500b1d8e8ef31e21c99d1db9a6444d3adf1270", symbol: "WMATIC", coinGeckoId: "matic-network", decimals: 8, price: 0.941541},           // Addr: 0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270, Notional: 4306032
		tokenConfigEntry{chain: 5, addr: "0000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa84174", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                  // Addr: 0x2791bca1f2de4661ed88a30c99a7a9449aa84174, Notional: 9629116
		tokenConfigEntry{chain: 5, addr: "0000000000000000000000007ceb23fd6bc0add59e62ac25578270cff1b9f619", symbol: "WETH", coinGeckoId: "weth", decimals: 8, price: 1881.9},                        // Addr: 0x7ceb23fd6bc0add59e62ac25578270cff1b9f619, Notional: 47197
		tokenConfigEntry{chain: 5, addr: "000000000000000000000000c2132d05d31c914a87c6611c10748aeb04b58e8f", symbol: "USDT", coinGeckoId: "tether", decimals: 6, price: 0.999273},                    // Addr: 0xc2132d05d31c914a87c6611c10748aeb04b58e8f, Notional: 3159634
		tokenConfigEntry{chain: 6, addr: "0000000000000000000000002b2c81e08f1af8835a78bb2a90ae924ace0ea4be", symbol: "sAVAX", coinGeckoId: "benqi-liquid-staked-avax", decimals: 8, price: 28.34},    // Addr: 0x2b2c81e08f1af8835a78bb2a90ae924ace0ea4be, Notional: 802854
		tokenConfigEntry{chain: 6, addr: "0000000000000000000000009702230a8ea53601f5cd2dc00fdbc13d4df4a8c7", symbol: "USDt", coinGeckoId: "tether", decimals: 6, price: 0.999273},                    // Addr: 0x9702230a8ea53601f5cd2dc00fdbc13d4df4a8c7, Notional: 3145486
		tokenConfigEntry{chain: 6, addr: "000000000000000000000000a7d7079b0fead91f3e65f86e8915cb59c1a4c664", symbol: "USDC.e", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                // Addr: 0xa7d7079b0fead91f3e65f86e8915cb59c1a4c664, Notional: 70738
		tokenConfigEntry{chain: 6, addr: "000000000000000000000000b31f66aa3c1e785363f0875a1b74e27b85fd66c7", symbol: "WAVAX", coinGeckoId: "avalanche-2", decimals: 8, price: 27.5},                  // Addr: 0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7, Notional: 6035125
		tokenConfigEntry{chain: 6, addr: "000000000000000000000000b97ef9ef8734c71904d8002f8b6bc66dd9c48a6e", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                  // Addr: 0xb97ef9ef8734c71904d8002f8b6bc66dd9c48a6e, Notional: 3778591
		tokenConfigEntry{chain: 7, addr: "00000000000000000000000021c718c22d52d0f3a789b752d4c2fd5908a8a733", symbol: "wROSE", coinGeckoId: "oasis-network", decimals: 8, price: 0.092342},            // Addr: 0x21c718c22d52d0f3a789b752d4c2fd5908a8a733, Notional: 250814
		tokenConfigEntry{chain: 7, addr: "00000000000000000000000094fbffe5698db6f54d6ca524dbe673a7729014be", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                  // Addr: 0x94fbffe5698db6f54d6ca524dbe673a7729014be, Notional: 42945
		tokenConfigEntry{chain: 8, addr: "0000000000000000000000000000000000000000000000000000000000000000", symbol: "ALGO", coinGeckoId: "algorand", decimals: 6, price: 0.356904},                  // *** manually added
		tokenConfigEntry{chain: 8, addr: "0000000000000000000000000000000000000000000000000000000001e1ab70", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                  // *** manually added
		tokenConfigEntry{chain: 8, addr: "000000000000000000000000000000000000000000000000000000000004c5c1", symbol: "USDT", coinGeckoId: "tether", decimals: 6, price: 0.999273},                    // *** manually added
		tokenConfigEntry{chain: 9, addr: "0000000000000000000000004988a896b1227218e4a686fde5eabdcabd91571f", symbol: "USDT", coinGeckoId: "tether", decimals: 6, price: 0.999273},                    // Addr: 0x4988a896b1227218e4a686fde5eabdcabd91571f, Notional: 655292
		tokenConfigEntry{chain: 9, addr: "0000000000000000000000005183e1b1091804bc2602586919e6880ac1cf2896", symbol: "USN", coinGeckoId: "usn", decimals: 8, price: 0.999954},                        // Addr: 0x5183e1b1091804bc2602586919e6880ac1cf2896, Notional: 17
		tokenConfigEntry{chain: 9, addr: "000000000000000000000000b12bfca5a55806aaf64e99521918a4bf0fc40802", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                  // Addr: 0xb12bfca5a55806aaf64e99521918a4bf0fc40802, Notional: 643075
		tokenConfigEntry{chain: 9, addr: "000000000000000000000000c4bdd27c33ec7daa6fcfd8532ddb524bf4038096", symbol: "atLUNA", coinGeckoId: "wrapped-terra", decimals: 8, price: 0.0001085},          // Addr: 0xc4bdd27c33ec7daa6fcfd8532ddb524bf4038096, Notional: 0
		tokenConfigEntry{chain: 9, addr: "000000000000000000000000c9bdeed33cd01541e1eed10f90519d2c06fe3feb", symbol: "WETH", coinGeckoId: "weth", decimals: 8, price: 1881.9},                        // Addr: 0xc9bdeed33cd01541e1eed10f90519d2c06fe3feb, Notional: 4978
		tokenConfigEntry{chain: 9, addr: "0000000000000000000000008bec47865ade3b172a928df8f990bc7f2a3b9f79", symbol: "AURORA", coinGeckoId: "aurora", decimals: 8, price: 1.82},                      // *** manually added
		tokenConfigEntry{chain: 9, addr: "000000000000000000000000e4b9e004389d91e4134a28f19bd833cba1d994b6", symbol: "FRAX", coinGeckoId: "frax", decimals: 8, price: 0.999322},                      // *** manually added
		tokenConfigEntry{chain: 9, addr: "000000000000000000000000c42c30ac6cc15fac9bd938618bcaa1a1fae8501d", symbol: "NEAR", coinGeckoId: "near", decimals: 8, price: 4.330},                         // *** manually added. Near on aurora. 24 decimals
		tokenConfigEntry{chain: 10, addr: "00000000000000000000000004068da6c83afcfa0e13ba15a6696662335d5b75", symbol: "USDC", coinGeckoId: "usd-coin", decimals: 6, price: 0.999041},                 // Addr: 0x04068da6c83afcfa0e13ba15a6696662335d5b75, Notional: 778356
		tokenConfigEntry{chain: 10, addr: "00000000000000000000000021be370d5312f44cb42ce377bc9b8a0cef1a4c83", symbol: "WFTM", coinGeckoId: "wrapped-fantom", decimals: 8, price: 0.370202},           // Addr: 0x21be370d5312f44cb42ce377bc9b8a0cef1a4c83, Notional: 126455
		tokenConfigEntry{chain: 10, addr: "000000000000000000000000260b3e40c714ce8196465ec824cd8bb915081812", symbol: "IronICE", coinGeckoId: "iron-bsc", decimals: 8, price: 0.571011},              // Addr: 0x260b3e40c714ce8196465ec824cd8bb915081812, Notional: 1303
		tokenConfigEntry{chain: 10, addr: "000000000000000000000000321162cd933e2be498cd2267a90534a804051b11", symbol: "BTC", coinGeckoId: "wrapped-bitcoin", decimals: 8, price: 23944},              // Addr: 0x321162cd933e2be498cd2267a90534a804051b11, Notional: 2817
		tokenConfigEntry{chain: 10, addr: "00000000000000000000000074b23882a30290451a17c44f4f05243b6b58c76d", symbol: "ETH", coinGeckoId: "weth", decimals: 8, price: 1881.9},                        // Addr: 0x74b23882a30290451a17c44f4f05243b6b58c76d, Notional: 3154
		tokenConfigEntry{chain: 11, addr: "0000000000000000000000000000000000000000000100000000000000000080", symbol: "KAR", coinGeckoId: "karura", decimals: 8, price: 0.514157},                    // Addr: 0x0000000000000000000100000000000000000080, Notional: 0
		tokenConfigEntry{chain: 11, addr: "0000000000000000000000000000000000000000000100000000000000000081", symbol: "aUSD", coinGeckoId: "acala-dollar", decimals: 8, price: 0.922336},             // Addr: 0x0000000000000000000100000000000000000081, Notional: 15
		tokenConfigEntry{chain: 11, addr: "0000000000000000000000000000000000000000000500000000000000000007", symbol: "USDT", coinGeckoId: "tether", decimals: 6, price: 0.999273},                   // Addr: 0x0000000000000000000500000000000000000007, Notional: 125391
		tokenConfigEntry{chain: 11, addr: "0000000000000000000000000000000000000000000100000000000000000082", symbol: "KSM", coinGeckoId: "kusama", decimals: 8, price: 56.34},                       // *** manually added
		tokenConfigEntry{chain: 12, addr: "0000000000000000000000000000000000000000000100000000000000000001", symbol: "aUSD", coinGeckoId: "acala-dollar", decimals: 8, price: 0.922336},             // Addr: 0x0000000000000000000100000000000000000001, Notional: 807
		tokenConfigEntry{chain: 12, addr: "0000000000000000000000000000000000000000000100000000000000000000", symbol: "ACA", coinGeckoId: "acala", decimals: 8, price: 0.271192},                     // *** manually added
		tokenConfigEntry{chain: 12, addr: "0000000000000000000000000000000000000000000100000000000000000002", symbol: "DOT", coinGeckoId: "polkadot", decimals: 8, price: 8.85},                      // *** manually added
		tokenConfigEntry{chain: 13, addr: "000000000000000000000000e4f05a66ec68b54a58b17c22107b02e0232cc817", symbol: "WKLAY", coinGeckoId: "klay-token", decimals: 8, price: 0.296369},              // Addr: 0xe4f05a66ec68b54a58b17c22107b02e0232cc817, Notional: 1
		tokenConfigEntry{chain: 13, addr: "0000000000000000000000005fff3a6c16c2208103f318f4713d4d90601a7313", symbol: "KLEVA", coinGeckoId: "kleva", decimals: 8, price: 0.198829},                   // *** manually added
		tokenConfigEntry{chain: 13, addr: "0000000000000000000000005096db80b21ef45230c9e423c373f1fc9c0198dd", symbol: "WEMIX", coinGeckoId: "wemix-token", decimals: 8, price: 2.72},                 // *** manually added
		tokenConfigEntry{chain: 13, addr: "0000000000000000000000005c74070fdea071359b86082bd9f9b3deaafbe32b", symbol: "KDAI", coinGeckoId: "dai", decimals: 8, price: 0.998668},                      // *** manually added
		tokenConfigEntry{chain: 13, addr: "000000000000000000000000cee8faf64bb97a73bb51e115aa89c17ffa8dd167", symbol: "oUSDT", coinGeckoId: "tether", decimals: 6, price: 0.999273},                  // *** manually added
		tokenConfigEntry{chain: 14, addr: "00000000000000000000000046c9757c5497c5b1f2eb73ae79b6b67d119b0b58", symbol: "PACT", coinGeckoId: "impactmarket", decimals: 8, price: 0.00125827},           // Addr: 0x46c9757c5497c5b1f2eb73ae79b6b67d119b0b58, Notional: 1698
		tokenConfigEntry{chain: 14, addr: "000000000000000000000000471ece3750da237f93b8e339c536989b8978a438", symbol: "CELO", coinGeckoId: "celo", decimals: 8, price: 1.059},                        // Addr: 0x471ece3750da237f93b8e339c536989b8978a438, Notional: 2006
		tokenConfigEntry{chain: 14, addr: "000000000000000000000000765de816845861e75a25fca122bb6898b8b1282a", symbol: "cUSD", coinGeckoId: "celo-dollar", decimals: 8, price: 0.993661},              // Addr: 0x765de816845861e75a25fca122bb6898b8b1282a, Notional: 1
		tokenConfigEntry{chain: 14, addr: "000000000000000000000000d8763cba276a3738e6de85b4b3bf5fded6d6ca73", symbol: "cEUR", coinGeckoId: "celo-euro", decimals: 8, price: 1.009},                   // Addr: 0xd8763cba276a3738e6de85b4b3bf5fded6d6ca73, Notional: 101465
		tokenConfigEntry{chain: 15, addr: "0000000000000000000000000000000000000000000000000000000000000000", symbol: "NEAR", coinGeckoId: "near", decimals: 8, price: 4.330},                        // *** manually added
		tokenConfigEntry{chain: 18, addr: "01fa6c6fbc36d8c245b0a852a43eb5d644e8b4c477b27bfab9537c10945939da", symbol: "LUNA", coinGeckoId: "terra-luna-2", decimals: 6, price: 1.99},                 // Addr: uluna, Notional: 1182
	}
}
