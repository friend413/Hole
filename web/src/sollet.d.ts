declare module '@project-serum/sol-wallet-adapter' {
    import EventEmitter = NodeJS.EventEmitter;
    import {PublicKey, Transaction} from "@solana/web3.js";

    export default class Wallet extends EventEmitter {
        public publicKey: PublicKey;

        constructor(url: string, network: string);

        async connect();
        async disconnect();
        async signTransaction(tx: Transaction): Promise<Transaction>;
    }
}
