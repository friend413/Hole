require('dotenv').config({ path: ".env" });
const HDWalletProvider = require('@truffle/hdwallet-provider');

module.exports = {
    networks: {
        development: {
            host: "127.0.0.1",
            port: 8545,
            network_id: "*",
        },
        rinkeby: {
            provider: () => new HDWalletProvider(process.env.MNEMONIC, `https://rinkeby.infura.io/v3/`+process.env.INFURA_KEY),
            network_id: 4,
            gas: 5500000,
            confirmations: 2,
            timeoutBlocks: 200,
            skipDryRun: true
        },
        goerli: {
            provider: () => {
                return new HDWalletProvider(process.env.MNEMONIC, 'https://goerli.infura.io/v3/'+process.env.INFURA_KEY)
            },
            network_id: '5',
            gas: 4465030,
            gasPrice: 10000000000,
        },
    },

    compilers: {
        solc: {
            version: "0.8.4",
            settings: {
                 optimizer: {
                   enabled: true,
                   runs: 200
                 },
            }
        }
    },

    plugins: [
        "@chainsafe/truffle-plugin-abigen",
        "truffle-plugin-verify"
    ],

    api_keys: {
        etherscan: process.env.ETHERSCAN_KEY
    },
};
