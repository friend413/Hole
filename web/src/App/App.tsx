import React, {useEffect, useMemo, useState} from 'react';
import './App.css';
import * as solanaWeb3 from '@solana/web3.js';
import ClientContext from '../providers/ClientContext';
import Transfer from "../pages/Transfer";
import {Empty, Layout} from 'antd';
import {SolanaTokenProvider} from "../providers/SolanaTokenContext";
import {SlotProvider} from "../providers/SlotContext";
import { HashRouter, Link, Route, Switch} from 'react-router-dom';
import TransferSolana from "../pages/TransferSolana";
import WalletContext from '../providers/WalletContext';
import Wallet from "@project-serum/sol-wallet-adapter";
import {BridgeProvider} from "../providers/BridgeContext";
import Assistant from "../pages/Assistant";
import {SOLANA_HOST} from "../config";

const {Header, Content, Footer} = Layout;

function App() {
    let c = new solanaWeb3.Connection(SOLANA_HOST);
    const wallet = useMemo(() => new Wallet("https://www.sollet.io", SOLANA_HOST), []);
    const [connected, setConnected] = useState(false);
    useEffect(() => {
        wallet.on('connect', () => {
            setConnected(true);
            console.log('Connected to wallet ' + wallet.publicKey.toBase58());
        });
        wallet.on('disconnect', () => {
            setConnected(false);
            console.log('Disconnected from wallet');
        });
        return () => {
            wallet.disconnect();
        };
    }, [wallet]);

    return (
        <div className="App">
            <Layout style={{height: '100%'}}>
                <HashRouter basename={"/"}>
                    <Header style={{position: 'fixed', zIndex: 1, width: '100%'}}>
                        <Link to="/" style={{paddingRight: 20}}>Assistant</Link>
                        <Link to="/eth" style={{paddingRight: 20}}>Ethereum</Link>
                        <Link to="/solana">Solana</Link>
                        {
                            connected ? (<a style={{float: "right"}}>
                                Connected as {wallet.publicKey.toString()}
                            </a>) : (<a style={{float: "right"}} onClick={() => {
                                if (!connected) {
                                    wallet.connect()
                                }
                            }
                            }>Connect</a>)
                        }

                    </Header>
                    <Content style={{padding: '0 50px', marginTop: 64}}>
                        <div style={{padding: 24}}>
                            {
                                connected?( <ClientContext.Provider value={c}>
                                    <SlotProvider>
                                        <WalletContext.Provider value={wallet}>
                                            <BridgeProvider>
                                                <SolanaTokenProvider>
                                                    <Switch>
                                                        <Route path="/">
                                                            <Assistant/>
                                                        </Route>
                                                        <Route path="/solana">
                                                            <TransferSolana/>
                                                        </Route>
                                                        <Route path="/eth">
                                                            <Transfer/>
                                                        </Route>
                                                    </Switch>
                                                </SolanaTokenProvider>
                                            </BridgeProvider>
                                        </WalletContext.Provider>
                                    </SlotProvider>
                                </ClientContext.Provider>):(
                                    <Empty description={"Please connect your wallet"}/>
                                )
                            }

                        </div>
                    </Content>
                    <Footer style={{textAlign: 'center'}}>The Wormhole Project</Footer>
                </HashRouter>
            </Layout>
        </div>
    );
}

export default App;
