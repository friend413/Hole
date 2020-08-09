import React, {useContext, useEffect, useState} from 'react';
import ClientContext from "../providers/ClientContext";
import * as solanaWeb3 from '@solana/web3.js';
import {PublicKey} from '@solana/web3.js';
import {Button, Form, Input, InputNumber, message, Select, Space} from "antd";
import {ethers} from "ethers";
import {Erc20Factory} from "../contracts/Erc20Factory";
import {Arrayish, BigNumber, BigNumberish} from "ethers/utils";
import {WormholeFactory} from "../contracts/WormholeFactory";
import {WrappedAssetFactory} from "../contracts/WrappedAssetFactory";
import {SolanaBridge} from "../utils/bridge";


// @ts-ignore
window.ethereum.enable();
// @ts-ignore
const provider = new ethers.providers.Web3Provider(window.ethereum);
const signer = provider.getSigner();

async function lockAssets(asset: string,
                          amount: BigNumberish,
                          recipient: Arrayish,
                          target_chain: BigNumberish) {
    let wh = WormholeFactory.connect("0xac3eB48829fFC3C37437ce4459cE63F1F4d4E0b4", signer);
    try {
        message.loading({content: "Signing transaction...", key: "eth_tx", duration: 1000},)
        let res = await wh.lockAssets(asset, amount, recipient, target_chain)
        message.loading({content: "Waiting for transaction to be mined...", key: "eth_tx", duration: 1000})
        await res.wait(1);
        message.success({content: "Transfer on ETH succeeded!", key: "eth_tx"})
    } catch (e) {
        message.error({content: "Transfer failed", key: "eth_tx"})
    }
}

async function approveAssets(asset: string,
                             amount: BigNumberish) {
    let e = Erc20Factory.connect(asset, signer);
    try {
        message.loading({content: "Signing transaction...", key: "eth_tx", duration: 1000})
        let res = await e.approve("0xac3eB48829fFC3C37437ce4459cE63F1F4d4E0b4", amount)
        message.loading({content: "Waiting for transaction to be mined...", key: "eth_tx", duration: 1000})
        await res.wait(1);
        message.success({content: "Approval on ETH succeeded!", key: "eth_tx"})
    } catch (e) {
        message.error({content: "Approval failed", key: "eth_tx"})
    }
}

function Transfer() {
    let c = useContext<solanaWeb3.Connection>(ClientContext);

    let [slot, setSlot] = useState(0);
    useEffect(() => {
        c.onSlotChange(value => {
            setSlot(value.slot);
        });
    })

    let [coinInfo, setCoinInfo] = useState({
        balance: new BigNumber(0),
        decimals: 0,
        allowance: new BigNumber(0),
        isWrapped: false,
        chainID: 0,
        wrappedAddress: ""
    });
    let [amount, setAmount] = useState(0);
    let [address, setAddress] = useState("");
    let [addressValid, setAddressValid] = useState(false)

    useEffect(() => {
        fetchBalance(address)
    }, [address])



    async function fetchBalance(token: string) {
        let p = new SolanaBridge(new PublicKey("FHbUryAag7ZfkFKbaCZaqWYsRgEtu7EWFrniy3VQ9Z3w"), new PublicKey("FHbUryAag7ZfkFKbaCZaqWYsRgEtu7EWFrniy3VQ9Z3w"), new PublicKey("FHbUryAag7ZfkFKbaCZaqWYsRgEtu7EWFrniy3VQ9Z3w"))
        console.log(p.programID.toBuffer())
        console.log(await p.createWrappedAsset(new PublicKey("FHbUryAag7ZfkFKbaCZaqWYsRgEtu7EWFrniy3VQ9Z3w"), 2000, {
            chain: 200,

            address: Buffer.of(1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
        }))
        try {
            let e = WrappedAssetFactory.connect(token, provider);
            let addr = await signer.getAddress();
            let balance = await e.balanceOf(addr);
            let decimals = await e.decimals();
            let allowance = await e.allowance(addr, "0xac3eB48829fFC3C37437ce4459cE63F1F4d4E0b4");

            let info = {
                balance: balance.div(new BigNumber(10).pow(decimals)),
                allowance: allowance.div(new BigNumber(10).pow(decimals)),
                decimals: decimals,
                isWrapped: false,
                chainID: 0,
                wrappedAddress: ""
            }

            let b = WormholeFactory.connect("0xac3eB48829fFC3C37437ce4459cE63F1F4d4E0b4", provider);

            let isWrapped = await b.isWrappedAsset(token)
            if (isWrapped) {
                info.chainID = await e.assetChain()
                info.wrappedAddress = await e.assetAddress()
                info.isWrapped = true
            }
            setCoinInfo(info)
            setAddressValid(true)
        } catch (e) {
            setAddressValid(false)
        }
    }

    return (
        <>
            <p>Slot: {slot}</p>
            <Space>
                <Form onFinish={(values) => {
                    let recipient = new solanaWeb3.PublicKey(values["recipient"]).toBuffer()
                    let transferAmount = new BigNumber(values["amount"]).mul(new BigNumber(10).pow(coinInfo.decimals));
                    if (coinInfo.allowance.toNumber() >= amount || coinInfo.isWrapped) {
                        lockAssets(values["address"], transferAmount, recipient, values["target_chain"])
                    } else {
                        approveAssets(values["address"], transferAmount)
                    }
                }}>
                    <Form.Item name="address" validateStatus={addressValid ? "success" : "error"}>
                        <Input addonAfter={`Balance: ${coinInfo.balance}`} name="address" placeholder={"ERC20 address"}
                               onBlur={(v) => {
                                   setAddress(v.target.value)
                               }}/>
                    </Form.Item>
                    <Form.Item name="amount" rules={[{
                        required: true, validator: (rule, value, callback) => {
                            let big = new BigNumber(value);
                            callback(big.lte(coinInfo.balance) ? undefined : "Amount exceeds balance")
                        }
                    }]}>
                        <InputNumber name={"amount"} placeholder={"Amount"} type={"number"} onChange={value => {
                            // @ts-ignore
                            setAmount(value || 0)
                        }}/>
                    </Form.Item>
                    <Form.Item name="target_chain" rules={[{required: true, message: "Please choose a target chain"}]}>
                        <Select placeholder="Target Chain">
                            <Select.Option value={1}>
                                Solana
                            </Select.Option>
                        </Select>
                    </Form.Item>
                    <Form.Item name="recipient" rules={[{
                        required: true,
                        validator: (rule, value, callback) => {
                            try {
                                new solanaWeb3.PublicKey(value);
                                callback();
                            } catch (e) {
                                callback("Not a valid Solana address");
                            }
                        }
                    },]}>
                        <Input name="recipient" placeholder={"Address of the recipient"}/>
                    </Form.Item>
                    <Form.Item>
                        <Button type="primary" htmlType="submit">
                            {coinInfo.allowance.toNumber() >= amount || coinInfo.isWrapped ? "Transfer" : "Approve"}
                        </Button>
                    </Form.Item>
                </Form>
            </Space>
        </>
    );
}

export default Transfer;
