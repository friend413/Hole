mod helpers;

use accounting::state::{
    account::{self, Balance},
    transfer, Account, TokenAddress,
};
use cosmwasm_std::{to_binary, Binary, Event, Uint256};
use cw_multi_test::AppResponse;
use helpers::*;
use wormchain_accounting::msg::Observation;
use wormhole::{token::Message, Address, Amount};
use wormhole_bindings::fake;

fn set_up(count: usize) -> (Vec<Message>, Vec<Observation>) {
    let mut txs = Vec::with_capacity(count);
    let mut observations = Vec::with_capacity(count);
    for i in 0..count {
        let key = transfer::Key::new(i as u16, [i as u8; 32].into(), i as u64);
        let tx = Message::Transfer {
            amount: Amount(Uint256::from(500u128).to_be_bytes()),
            token_address: Address([(i + 1) as u8; 32]),
            token_chain: (i as u16).into(),
            recipient: Address([(i + 2) as u8; 32]),
            recipient_chain: ((i + 3) as u16).into(),
            fee: Amount([0u8; 32]),
        };
        let payload = serde_wormhole::to_vec(&tx).map(Binary::from).unwrap();
        txs.push(tx);
        observations.push(Observation {
            key,
            nonce: i as u32,
            tx_hash: vec![(i + 4) as u8; 20].into(),
            payload,
        });
    }

    (txs, observations)
}

#[test]
fn batch() {
    const COUNT: usize = 5;

    let (txs, observations) = set_up(COUNT);
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());

    let index = wh.guardian_set_index();

    let obs = to_binary(&observations).unwrap();
    let signatures = wh.sign(&obs);
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    for (i, s) in signatures.into_iter().enumerate() {
        if i < quorum {
            contract.submit_observations(obs.clone(), index, s).unwrap();

            // Once there is a quorum the pending transfers are removed.
            if i < quorum - 1 {
                for o in &observations {
                    let data = contract.query_pending_transfer(o.key.clone()).unwrap();
                    assert_eq!(o, data[0].observation());

                    // Make sure the transfer hasn't yet been committed.
                    contract
                        .query_transfer(o.key.clone())
                        .expect_err("transfer committed without quorum");
                }
            } else {
                for o in &observations {
                    contract
                        .query_pending_transfer(o.key.clone())
                        .expect_err("found pending transfer for observation with quorum");
                }
            }
        } else {
            contract
                .submit_observations(obs.clone(), index, s)
                .expect_err("successfully submitted observation for committed transfer");
        }
    }

    for (tx, o) in txs.into_iter().zip(observations) {
        let expected = if let Message::Transfer {
            amount,
            token_address,
            token_chain,
            recipient_chain,
            ..
        } = tx
        {
            transfer::Data {
                amount: Uint256::new(amount.0),
                token_chain: token_chain.into(),
                token_address: TokenAddress::new(token_address.0),
                recipient_chain: recipient_chain.into(),
            }
        } else {
            panic!("unexpected tokenbridge payload");
        };

        let emitter_chain = o.key.emitter_chain();
        let actual = contract.query_transfer(o.key).unwrap();
        assert_eq!(expected, actual);

        let src = contract
            .query_balance(account::Key::new(
                emitter_chain,
                expected.token_chain,
                expected.token_address,
            ))
            .unwrap();

        assert_eq!(expected.amount, *src);

        let dst = contract
            .query_balance(account::Key::new(
                expected.recipient_chain,
                expected.token_chain,
                expected.token_address,
            ))
            .unwrap();

        assert_eq!(expected.amount, *dst);
    }
}

#[test]
fn duplicates() {
    const COUNT: usize = 5;

    let (txs, observations) = set_up(COUNT);
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();

    let obs = to_binary(&observations).unwrap();
    let signatures = wh.sign(&obs);
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    for (i, s) in signatures.iter().take(quorum).cloned().enumerate() {
        contract.submit_observations(obs.clone(), index, s).unwrap();
        let err = contract
            .submit_observations(obs.clone(), index, s)
            .expect_err("successfully submitted duplicate observations");
        if i < quorum - 1 {
            // Sadly we can't match on the exact error type in an integration test because the
            // test frameworks converts it into a string before it reaches this point.
            assert!(format!("{err:#}").contains("duplicate signatures"));
        }
    }

    for (tx, o) in txs.into_iter().zip(observations) {
        let expected = if let Message::Transfer {
            amount,
            token_address,
            token_chain,
            recipient_chain,
            ..
        } = tx
        {
            transfer::Data {
                amount: Uint256::new(amount.0),
                token_chain: token_chain.into(),
                token_address: TokenAddress::new(token_address.0),
                recipient_chain: recipient_chain.into(),
            }
        } else {
            panic!("unexpected tokenbridge payload");
        };

        let emitter_chain = o.key.emitter_chain();
        let actual = contract.query_transfer(o.key).unwrap();
        assert_eq!(expected, actual);

        let src = contract
            .query_balance(account::Key::new(
                emitter_chain,
                expected.token_chain,
                expected.token_address,
            ))
            .unwrap();

        assert_eq!(expected.amount, *src);

        let dst = contract
            .query_balance(account::Key::new(
                expected.recipient_chain,
                expected.token_chain,
                expected.token_address,
            ))
            .unwrap();

        assert_eq!(expected.amount, *dst);
    }

    for s in signatures {
        contract
            .submit_observations(obs.clone(), index, s)
            .expect_err("successfully submitted observation for committed transfer");
    }
}

fn transfer_tokens(
    wh: &fake::WormholeKeeper,
    contract: &mut Contract,
    key: transfer::Key,
    msg: Message,
    index: u32,
    quorum: usize,
) -> anyhow::Result<(Observation, Vec<AppResponse>)> {
    let payload = serde_wormhole::to_vec(&msg).map(Binary::from).unwrap();
    let o = Observation {
        key,
        nonce: 0x4343b191,
        tx_hash: vec![0xd8u8; 20].into(),
        payload,
    };

    let obs = to_binary(&vec![o.clone()]).unwrap();
    let signatures = wh.sign(&obs);

    let responses = signatures
        .into_iter()
        .take(quorum)
        .map(|s| contract.submit_observations(obs.clone(), index, s))
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok((o, responses))
}

#[test]
fn round_trip() {
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let emitter_chain = 2;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = Address([0xccu8; 32]);
    let token_chain = 2u16.into();
    let recipient_chain = 14u16.into();

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain,
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(&wh, &mut contract, key.clone(), msg, index, quorum).unwrap();

    let expected = transfer::Data {
        amount: Uint256::new(amount.0),
        token_chain: token_chain.into(),
        token_address: TokenAddress::new(token_address.0),
        recipient_chain: recipient_chain.into(),
    };
    let actual = contract.query_transfer(key).unwrap();
    assert_eq!(expected, actual);

    // Now send the tokens back.
    let key = transfer::Key::new(
        recipient_chain.into(),
        [u16::from(recipient_chain) as u8; 32].into(),
        91156748,
    );
    let msg = Message::Transfer {
        amount,
        token_address,
        token_chain,
        recipient: Address([0xe4u8; 32]),
        recipient_chain: emitter_chain.into(),
        fee: Amount([0u8; 32]),
    };
    transfer_tokens(&wh, &mut contract, key.clone(), msg, index, quorum).unwrap();

    let expected = transfer::Data {
        amount: Uint256::new(amount.0),
        token_chain: token_chain.into(),
        token_address: TokenAddress::new(token_address.0),
        recipient_chain: emitter_chain,
    };
    let actual = contract.query_transfer(key).unwrap();
    assert_eq!(expected, actual);

    // Now both balances should be zero.
    let src = contract
        .query_balance(account::Key::new(
            emitter_chain,
            token_chain.into(),
            expected.token_address,
        ))
        .unwrap();

    assert_eq!(Uint256::zero(), *src);

    let dst = contract
        .query_balance(account::Key::new(
            recipient_chain.into(),
            token_chain.into(),
            expected.token_address,
        ))
        .unwrap();

    assert_eq!(Uint256::zero(), *dst);
}

#[test]
fn missing_guardian_set() {
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let emitter_chain = 2;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = Address([0xccu8; 32]);
    let token_chain = 2.into();
    let recipient_chain = 14.into();

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain,
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(&wh, &mut contract, key, msg, index + 1, quorum)
        .expect_err("successfully submitted observations with invalid guardian set");
}

#[test]
fn expired_guardian_set() {
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let mut block = contract.app().block_info();

    let quorum = wh.calculate_quorum(index, block.height).unwrap() as usize;

    let emitter_chain = 2;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = Address([0xccu8; 32]);
    let token_chain = 2.into();
    let recipient_chain = 14.into();

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain,
        fee: Amount([0u8; 32]),
    };

    // Mark the guardian set expired.
    wh.set_expiration(block.height);
    block.height += 1;
    contract.app_mut().set_block(block);

    transfer_tokens(&wh, &mut contract, key, msg, index, quorum)
        .expect_err("successfully submitted observations with expired guardian set");
}

#[test]
fn no_quorum() {
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let emitter_chain = 2;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = Address([0xccu8; 32]);
    let token_chain = 2.into();
    let recipient_chain = 14.into();

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain,
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(
        &wh,
        &mut contract,
        key.clone(),
        msg.clone(),
        index,
        quorum - 1,
    )
    .unwrap();

    let data = contract.query_pending_transfer(key.clone()).unwrap();
    assert_eq!(key, data[0].observation().key);

    let actual = serde_wormhole::from_slice(&data[0].observation().payload).unwrap();
    assert_eq!(msg, actual);

    // Make sure the transfer hasn't yet been committed.
    contract
        .query_transfer(key)
        .expect_err("transfer committed without quorum");
}

#[test]
fn missing_wrapped_account() {
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let emitter_chain = 14;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = Address([0xccu8; 32]);
    let token_chain = 2.into();
    let recipient_chain = 2.into();

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain,
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(&wh, &mut contract, key, msg, index, quorum)
        .expect_err("successfully burned wrapped tokens without a wrapped amount");
}

#[test]
fn missing_native_account() {
    let emitter_chain = 14;
    let recipient_chain = 2;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = [0xccu8; 32];
    let token_chain = 2;

    // We need to set up a fake wrapped account so that the initial check succeeds.
    let (wh, mut contract) = proper_instantiate(
        vec![Account {
            key: account::Key::new(emitter_chain, token_chain, token_address.into()),
            balance: Balance::new(Uint256::new(amount.0)),
        }],
        Vec::new(),
        Vec::new(),
    );
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address: Address(token_address),
        token_chain: token_chain.into(),
        recipient: Address([0xb9u8; 32]),
        recipient_chain: recipient_chain.into(),
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(&wh, &mut contract, key, msg, index, quorum)
        .expect_err("successfully unlocked native tokens without a native account");
}

#[test]
fn repeated() {
    const ITERATIONS: usize = 10;

    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let emitter_chain = 2;
    let recipient_chain = 14;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = [0xccu8; 32];
    let token_chain = 2;

    let msg = Message::Transfer {
        amount,
        token_address: Address(token_address),
        token_chain: token_chain.into(),
        recipient: Address([0xb9u8; 32]),
        recipient_chain: recipient_chain.into(),
        fee: Amount([0u8; 32]),
    };

    for i in 0..ITERATIONS {
        let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), i as u64);
        transfer_tokens(&wh, &mut contract, key.clone(), msg.clone(), index, quorum).unwrap();
    }

    let expected = Uint256::new(amount.0) * Uint256::from(ITERATIONS as u128);
    let src = contract
        .query_balance(account::Key::new(
            emitter_chain,
            token_chain,
            token_address.into(),
        ))
        .unwrap();

    assert_eq!(expected, *src);

    let dst = contract
        .query_balance(account::Key::new(
            recipient_chain,
            token_chain,
            token_address.into(),
        ))
        .unwrap();

    assert_eq!(expected, *dst);
}

#[test]
fn wrapped_to_wrapped() {
    let emitter_chain = 14;
    let recipient_chain = 2;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = [0xccu8; 32];
    let token_chain = 5;

    // We need an initial fake wrapped account.
    let (wh, mut contract) = proper_instantiate(
        vec![Account {
            key: account::Key::new(emitter_chain, token_chain, token_address.into()),
            balance: Balance::new(Uint256::new(amount.0)),
        }],
        Vec::new(),
        Vec::new(),
    );
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address: Address(token_address),
        token_chain: token_chain.into(),
        recipient: Address([0xb9u8; 32]),
        recipient_chain: recipient_chain.into(),
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(&wh, &mut contract, key.clone(), msg, index, quorum).unwrap();

    let expected = transfer::Data {
        amount: Uint256::new(amount.0),
        token_chain,
        token_address: TokenAddress::new(token_address),
        recipient_chain,
    };
    let actual = contract.query_transfer(key).unwrap();
    assert_eq!(expected, actual);

    let src = contract
        .query_balance(account::Key::new(
            emitter_chain,
            token_chain,
            token_address.into(),
        ))
        .unwrap();

    assert_eq!(Uint256::zero(), *src);

    let dst = contract
        .query_balance(account::Key::new(
            recipient_chain,
            token_chain,
            token_address.into(),
        ))
        .unwrap();

    assert_eq!(Uint256::new(amount.0), *dst);
}

#[test]
fn unknown_emitter() {
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let emitter_chain = 14;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = Address([0xccu8; 32]);
    let token_chain = 2.into();
    let recipient_chain = 2.into();

    let key = transfer::Key::new(emitter_chain, [0xde; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain,
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(&wh, &mut contract, key, msg, index, quorum)
        .expect_err("successfully transfered tokens with an invalid emitter address");
}

#[test]
fn different_observations() {
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    // First submit some observations without enough signatures for quorum.
    let emitter_chain = 2;
    let fake_amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = Address([0xccu8; 32]);
    let token_chain = 2.into();
    let fake_recipient_chain = 14.into();

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let fake = Message::Transfer {
        amount: fake_amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain: fake_recipient_chain,
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(&wh, &mut contract, key.clone(), fake, index, quorum - 1).unwrap();

    // Make sure there is no committed transfer yet.
    contract
        .query_transfer(key.clone())
        .expect_err("committed transfer without quorum");

    // Now change the details of the transfer and resubmit with the same key.
    let real_amount = Amount(Uint256::from(200u128).to_be_bytes());
    let real_recipient_chain = 9.into();
    let real = Message::Transfer {
        amount: real_amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain: real_recipient_chain,
        fee: Amount([0u8; 32]),
    };

    transfer_tokens(&wh, &mut contract, key.clone(), real, index, quorum).unwrap();

    contract
        .query_pending_transfer(key.clone())
        .expect_err("found pending transfer for observation with quorum");

    let expected = transfer::Data {
        amount: Uint256::new(real_amount.0),
        token_chain: token_chain.into(),
        token_address: TokenAddress::new(token_address.0),
        recipient_chain: real_recipient_chain.into(),
    };
    let actual = contract.query_transfer(key).unwrap();
    assert_eq!(expected, actual);

    let src = contract
        .query_balance(account::Key::new(
            emitter_chain,
            token_chain.into(),
            expected.token_address,
        ))
        .unwrap();

    assert_eq!(Uint256::new(real_amount.0), *src);

    let dst = contract
        .query_balance(account::Key::new(
            real_recipient_chain.into(),
            token_chain.into(),
            expected.token_address,
        ))
        .unwrap();

    assert_eq!(Uint256::new(real_amount.0), *dst);
}

#[test]
fn emit_event_with_quorum() {
    let (wh, mut contract) = proper_instantiate(Vec::new(), Vec::new(), Vec::new());
    let index = wh.guardian_set_index();
    let quorum = wh
        .calculate_quorum(index, contract.app().block_info().height)
        .unwrap() as usize;

    let emitter_chain = 2;
    let amount = Amount(Uint256::from(500u128).to_be_bytes());
    let token_address = Address([0xccu8; 32]);
    let token_chain = 2.into();
    let recipient_chain = 14.into();

    let key = transfer::Key::new(emitter_chain, [emitter_chain as u8; 32].into(), 37);
    let msg = Message::Transfer {
        amount,
        token_address,
        token_chain,
        recipient: Address([0xb9u8; 32]),
        recipient_chain,
        fee: Amount([0u8; 32]),
    };

    let (o, responses) = transfer_tokens(&wh, &mut contract, key, msg, index, quorum).unwrap();

    let expected = Event::new("wasm-Transfer")
        .add_attribute("emitter_chain", o.key.emitter_chain().to_string())
        .add_attribute("emitter_address", o.key.emitter_address().to_string())
        .add_attribute("sequence", o.key.sequence().to_string())
        .add_attribute("nonce", o.nonce.to_string())
        .add_attribute("tx_hash", o.tx_hash.to_base64())
        .add_attribute("payload", o.payload.to_base64());

    assert_eq!(responses.len(), quorum);
    for (i, r) in responses.into_iter().enumerate() {
        if i < quorum - 1 {
            assert!(!r.has_event(&expected));
        } else {
            r.assert_event(&expected);
        }
    }
}
