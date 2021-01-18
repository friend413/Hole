import React, {useContext, useEffect, useState} from 'react';
import ClientContext from "../providers/ClientContext";
import * as solanaWeb3 from '@solana/web3.js';
import {PublicKey, Transaction} from '@solana/web3.js';
import * as spl from '@solana/spl-token';
import {Button, Col, Form, Input, InputNumber, message, Row, Select} from "antd";
import {BigNumber} from "ethers/utils";
import SplBalances from "../components/SplBalances";
import {SlotContext} from "../providers/SlotContext";
import {SolanaTokenContext} from "../providers/SolanaTokenContext";
import {CHAIN_ID_SOLANA} from "../utils/bridge";
import {BridgeContext} from "../providers/BridgeContext";
import BN from 'bn.js';
import {TOKEN_PROGRAM} from "../config";
import WalletContext from "../providers/WalletContext";

function TransferSolana() {
    let c = useContext<solanaWeb3.Connection>(ClientContext);
    let slot = useContext(SlotContext);
    let b = useContext(SolanaTokenContext);
    let bridge = useContext(BridgeContext);
    let wallet = useContext(WalletContext);

    let [coinInfo, setCoinInfo] = useState({
        balance: new BigNumber(0),
        decimals: 0,
        isWrapped: false,
        chainID: 0,
        wrappedAddress: new Buffer([]),
        mint: ""
    });
    let [amount, setAmount] = useState(new BigNumber(0));
    let [address, setAddress] = useState("");
    let [addressValid, setAddressValid] = useState(false)

    useEffect(() => {
        async function getCoinInfo() {
            let acc = b.balances.find(value => value.account.toString() == address)
            if (!acc) {
                setAmount(new BigNumber(0));
                setAddressValid(false)
                return
            }

            setCoinInfo({
                balance: acc.balance,
                decimals: acc.decimals,
                isWrapped: acc.assetMeta.chain != CHAIN_ID_SOLANA,
                chainID: acc.assetMeta.chain,
                wrappedAddress: acc.assetMeta.address,
                mint: acc.mint
            })
            setAddressValid(true)
        }

        getCoinInfo()
    }, [address])


    return (
        <>
            <Row gutter={12}>
                <Col span={12}>
                    <p>Transfer from Solana:</p>
                    <Form onFinish={(values) => {
                        let recipient = new Buffer(values["recipient"].slice(2), "hex");

                        let transferAmount = new BN(values["amount"]).mul(new BN(10).pow(new BN(coinInfo.decimals)));
                        let fromAccount = new PublicKey(values["address"])

                        let send = async () => {
                            message.loading({content: "Transferring tokens...", key: "transfer"}, 1000)

                            let {ix: lock_ix} = await bridge.createLockAssetInstruction(wallet.publicKey, fromAccount, new PublicKey(coinInfo.mint), transferAmount, values["target_chain"], recipient,
                                {
                                    chain: coinInfo.chainID,
                                    address: coinInfo.wrappedAddress,
                                    decimals: Math.min(coinInfo.decimals, 9)
                                }, Math.random() * 100000);
                            let ix = spl.Token.createApproveInstruction(TOKEN_PROGRAM, fromAccount, await bridge.getConfigKey(), wallet.publicKey, [], transferAmount.toNumber())
                            let bridge_account = await bridge.getConfigKey();
                            let fee_ix = solanaWeb3.SystemProgram.transfer({
                                fromPubkey: wallet.publicKey,
                                toPubkey: bridge_account,
                                lamports: await bridge.getTransferFee()
                            });

                            let recentHash = await c.getRecentBlockhash();
                            let tx = new Transaction();
                            tx.recentBlockhash = recentHash.blockhash
                            tx.add(ix)
                            tx.add(fee_ix)
                            tx.add(lock_ix)
                            tx.feePayer = wallet.publicKey;
                            let signed = await wallet.signTransaction(tx)
                            try {
                                await c.sendRawTransaction(signed.serialize())
                                message.success({content: "Transfer succeeded", key: "transfer"})
                            } catch (e) {
                                message.error({content: "Transfer failed", key: "transfer"})
                            }
                        }
                        send()
                    }} layout={"vertical"}>
                        <Form.Item name="address" validateStatus={addressValid ? "success" : "error"}
                                   label={"Token Account:"}>
                            <Input
                                addonAfter={`Balance: ${coinInfo.balance.div(new BigNumber(Math.pow(10, coinInfo.decimals)))}`}
                                name="address"
                                placeholder={"Token account Pubkey"}
                                onBlur={(v) => {
                                    setAddress(v.target.value)
                                }}/>
                        </Form.Item>
                        <Form.Item name="amount" rules={[{
                            required: true, validator: (rule, value, callback) => {
                                let big = new BigNumber(value).mul(new BigNumber(10).pow(coinInfo.decimals));
                                callback(big.lte(coinInfo.balance) ? undefined : "Amount exceeds balance")
                            }
                        }]} label={"Amount:"}>
                            <InputNumber name={"amount"} placeholder={"Amount"} type={"number"} onChange={value => {
                                // @ts-ignore
                                setAmount(value || 0)
                            }}/>
                        </Form.Item>
                        <Form.Item name="target_chain"
                                   rules={[{required: true, message: "Please choose a target chain"}]}
                                   label={"Target Chain:"}>
                            <Select placeholder="Target Chain">
                                <Select.Option value={2}>
                                    Ethereum
                                </Select.Option>
                            </Select>
                        </Form.Item>
                        <Form.Item name="recipient" rules={[{
                            required: true,
                            validator: (rule, value, callback) => {
                                if (value.length !== 42 || value.indexOf("0x") != 0) {
                                    callback("Invalid address")
                                } else {
                                    callback()
                                }
                            }
                        }]} label={"Recipient:"}>
                            <Input name="recipient" placeholder={"Address of the recipient"}/>
                        </Form.Item>
                        <Form.Item>
                            <Button type="primary" htmlType="submit">
                                Transfer
                            </Button>
                        </Form.Item>
                    </Form>
                </Col>
            </Row>
            <Row>
                <Col>
                    <SplBalances/>
                </Col>
            </Row>
        </>
    );
}

export default TransferSolana;
