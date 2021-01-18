import React, {useContext, useEffect, useState} from 'react';
import ClientContext from "../providers/ClientContext";
import * as solanaWeb3 from '@solana/web3.js';
import {Account, Connection, PublicKey, Transaction} from '@solana/web3.js';
import {Button, Card, Col, Divider, Form, Input, InputNumber, List, message, Row, Select} from "antd";
import {ethers} from "ethers";
import {Erc20Factory} from "../contracts/Erc20Factory";
import {Arrayish, BigNumber, BigNumberish} from "ethers/utils";
import {WormholeFactory} from "../contracts/WormholeFactory";
import {WrappedAssetFactory} from "../contracts/WrappedAssetFactory";
import {BRIDGE_ADDRESS} from "../config";
import {SolanaTokenContext} from "../providers/SolanaTokenContext";
import {BridgeContext} from "../providers/BridgeContext";
import {AssetMeta, SolanaBridge} from "../utils/bridge";
import {FormInstance} from "antd/lib/form";
import SplBalances from "../components/SplBalances";
import TransferProposals from "../components/TransferProposals";
import WalletContext from "../providers/WalletContext";
import Wallet from "@project-serum/sol-wallet-adapter";

// @ts-ignore
if (window.ethereum === undefined) {
    alert("Please install the MetaMask extension before using this experimental demo web UI");
}

// @ts-ignore
window.ethereum.enable();
// @ts-ignore
const provider = new ethers.providers.Web3Provider(window.ethereum);
const signer = provider.getSigner();

async function lockAssets(asset: string,
                          amount: BigNumberish,
                          recipient: Arrayish,
                          target_chain: BigNumberish) {
    let wh = WormholeFactory.connect(BRIDGE_ADDRESS, signer);
    try {
        message.loading({content: "Signing transaction...", key: "eth_tx", duration: 1000},)
        let res = await wh.lockAssets(asset, amount, recipient, target_chain, 10, false)
        message.loading({content: "Waiting for transaction to be mined...", key: "eth_tx", duration: 1000})
        await res.wait(1);
        message.success({content: "Transfer on ETH succeeded!", key: "eth_tx"})
    } catch (e) {
        console.log(e)
        message.error({content: "Transfer failed", key: "eth_tx"})
    }
}

async function approveAssets(asset: string,
                             amount: BigNumberish) {
    let e = Erc20Factory.connect(asset, signer);
    try {
        message.loading({content: "Signing transaction...", key: "eth_tx", duration: 1000})
        let res = await e.approve(BRIDGE_ADDRESS, amount)
        message.loading({content: "Waiting for transaction to be mined...", key: "eth_tx", duration: 1000})
        await res.wait(1);
        message.success({content: "Approval on ETH succeeded!", key: "eth_tx"})
    } catch (e) {
        message.error({content: "Approval failed", key: "eth_tx"})
    }
}

async function createWrapped(c: Connection, b: SolanaBridge, wallet: Wallet, meta: AssetMeta, mint: PublicKey) {
    try {
        let tx = new Transaction();

        // @ts-ignore
        let [ix_account, newSigner] = await b.createWrappedAssetAndAccountInstructions(key.publicKey, mint, meta);
        let recentHash = await c.getRecentBlockhash();
        tx.recentBlockhash = recentHash.blockhash
        tx.add(...ix_account)
        tx.sign(newSigner)
        tx.feePayer = wallet.publicKey;
        let signed = await wallet.signTransaction(tx);
        message.loading({content: "Waiting for transaction to be confirmed...", key: "tx", duration: 1000})
        await c.sendRawTransaction(signed.serialize())
        message.success({content: "Creation succeeded!", key: "tx"})
    } catch (e) {
        console.log(e)
        message.error({content: "Creation failed", key: "tx"})
    }
}

function Transfer() {
    let c = useContext<solanaWeb3.Connection>(ClientContext);
    let tokenAccounts = useContext(SolanaTokenContext);
    let bridge = useContext(BridgeContext);
    let wallet = useContext(WalletContext);

    let [coinInfo, setCoinInfo] = useState({
        balance: new BigNumber(0),
        decimals: 0,
        allowance: new BigNumber(0),
        isWrapped: false,
        chainID: 0,
        assetAddress: new Buffer("")
    });
    let [amount, setAmount] = useState(new BigNumber(0));
    let [amountValid, setAmountValid] = useState(false);

    let [address, setAddress] = useState("");
    let [addressValid, setAddressValid] = useState(false)

    let [solanaAccount, setSolanaAccount] = useState({
        valid: false,
        message: ""
    })
    let [wrappedMint, setWrappedMint] = useState("")
    let [recipient, setRecipient] = useState("")

    let [transacting, setTransacting] = useState(false);

    let formRef = React.createRef<FormInstance>();

    useEffect(() => {
        let fetchBalance = async (token: string) => {
            try {
                let e = WrappedAssetFactory.connect(token, provider);
                let addr = await signer.getAddress();
                let balance = await e.balanceOf(addr);
                let decimals = await e.decimals();
                let allowance = await e.allowance(addr, BRIDGE_ADDRESS);

                let info = {
                    balance: balance.div(new BigNumber(10).pow(decimals)),
                    allowance: allowance.div(new BigNumber(10).pow(decimals)),
                    decimals: decimals,
                    isWrapped: false,
                    chainID: 2,
                    assetAddress: new Buffer(token.slice(2), "hex")
                }

                let b = WormholeFactory.connect(BRIDGE_ADDRESS, provider);

                let isWrapped = await b.isWrappedAsset(token)
                if (isWrapped) {
                    info.chainID = await e.assetChain()
                    info.assetAddress = new Buffer((await e.assetAddress()).slice(2), "hex")
                    info.isWrapped = true
                }
                setCoinInfo(info)
                setAddressValid(true)
            } catch (e) {
                setAddressValid(false)
            }
        }
        fetchBalance(address)
    }, [address, transacting])

    useEffect(() => {
        if (!addressValid) {
            setWrappedMint("")
            setSolanaAccount({
                valid: false,
                message: ""
            })
            return
        }

        let getWrappedInfo = async () => {
            let wrappedMint = await bridge.getWrappedAssetMint({
                chain: coinInfo.chainID,
                address: coinInfo.assetAddress,
                decimals: Math.min(coinInfo.decimals, 9)
            });
            setWrappedMint(wrappedMint.toString())

            for (let account of tokenAccounts.balances) {
                if (account.account.toString() == recipient) {
                    setSolanaAccount({
                        valid: true,
                        message: ""
                    })
                    return;
                }
            }
            setSolanaAccount({
                valid: false,
                message: "Not a valid wrapped token account"
            })
        }
        getWrappedInfo();
    }, [address, addressValid, tokenAccounts, bridge])

    useEffect(() => {
        setAmountValid(amount.lte(coinInfo.balance))
    }, [amount, coinInfo])

    return (
        <>
            <Row gutter={12}>
                <Col span={12}>
                    <Form onFinish={(values) => {
                        let recipient = new solanaWeb3.PublicKey(values["recipient"]).toBuffer()
                        let transferAmount = new BigNumber(values["amount"]).mul(new BigNumber(10).pow(coinInfo.decimals));
                        setTransacting(true)
                        if (coinInfo.allowance.gte(amount) || coinInfo.isWrapped) {
                            lockAssets(values["address"], transferAmount, recipient, values["target_chain"]).finally(() => {
                                setTransacting(false)
                            })
                        } else {
                            approveAssets(values["address"], transferAmount).finally(() => {
                                setTransacting(false)
                            })
                        }
                    }} style={{width: "100%"}} ref={formRef} layout={"vertical"}>
                        <Form.Item name="address" validateStatus={addressValid ? "success" : "error"} label={"Token:"}>
                            <Input addonAfter={`Balance: ${coinInfo.balance}`} name="address"
                                   placeholder={"ERC20 address"}
                                   onBlur={(v) => {
                                       setAddress(v.target.value)
                                   }}/>
                        </Form.Item>
                        <Form.Item name="amount" validateStatus={amountValid ? "success" : "error"} label={"Amount:"}>
                            <InputNumber name={"amount"} placeholder={"Amount"} type={"number"} min={0}
                                         onChange={value => {
                                             // @ts-ignore
                                             setAmount(new BigNumber(value) || 0)
                                         }}/>
                        </Form.Item>
                        <Form.Item name="target_chain"
                                   label={"Target Chain:"}>
                            <Select placeholder="Target Chain">
                                <Select.Option value={1}>
                                    Solana
                                </Select.Option>
                            </Select>
                        </Form.Item>
                        <Form.Item name="recipient" validateStatus={solanaAccount.valid ? "success" : "error"}
                                   help={recipient === "" ? undefined : solanaAccount.message} label={"Recipient"}>
                            <Input name="recipient" placeholder={"Address of the recipient"}
                                   onFocus={(v) => {
                                       setRecipient(v.target.value)
                                   }}/>
                        </Form.Item>
                        <Form.Item>
                            <Button type="primary" htmlType="submit" style={{marginLeft: "auto"}}
                                    disabled={transacting}>
                                {coinInfo.allowance.gte(amount) || coinInfo.isWrapped ? "Transfer" : "Approve"}
                            </Button>
                        </Form.Item>
                    </Form>
                </Col>
                <Col span={12}>
                    <Card>
                        <Row justify={"space-between"} align={"middle"}>
                            <Col>Token Accounts on Solana:</Col>
                            <Col><Button size={"small"}
                                         disabled={wrappedMint === ""}
                                         onClick={() => {
                                             createWrapped(c, bridge, wallet, {
                                                 chain: coinInfo.chainID,
                                                 address: coinInfo.assetAddress,
                                                 decimals: Math.min(coinInfo.decimals, 9)
                                             }, new PublicKey(wrappedMint))
                                         }}>Create new</Button></Col>
                        </Row>
                        <Divider/>
                        <Row>
                            <Col span={24}>
                                <List>
                                    {
                                        tokenAccounts.balances
                                            .filter(value => value.mint == wrappedMint)
                                            .map(v => (
                                                <List.Item
                                                    actions={[(<Button size={"small"} type={"dashed"} onClick={() => {
                                                        setRecipient(v.account.toString())
                                                        formRef.current?.setFieldsValue({
                                                            "recipient": v.account.toString()
                                                        })
                                                    }}>use</Button>)]}>
                                                    {v.account.toString()}
                                                </List.Item>
                                            ))
                                    }
                                </List>
                            </Col>
                        </Row>
                    </Card>
                </Col>
            </Row>
            <Row>
                <Col>
                    <SplBalances/>
                </Col>
            </Row>
            <Row>
                <Col>
                    <TransferProposals/>
                </Col>
            </Row>
        </>
    );
}

export default Transfer;
