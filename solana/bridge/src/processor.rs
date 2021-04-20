//! Program instruction processing logic
#![cfg(feature = "program")]

use std::{io::Write, mem::size_of, slice::Iter};

use byteorder::ByteOrder;
use num_traits::AsPrimitive;

use sha3::Digest;
use solana_program::program::invoke_signed;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{create_account},
    sysvar::Sysvar,
};

use crate::instruction::PublishMessagePayload;
use crate::vaa::{BodyContractUpgrade, BodyMessage};
use crate::{
    error::Error,
    instruction::{
        BridgeInstruction, BridgeInstruction::*, VAAData, VerifySigPayload, CHAIN_ID_SOLANA,
        MAX_LEN_GUARDIAN_KEYS, MAX_VAA_SIZE,
    },
    state::*,
    vaa::{BodyUpdateGuardianSet, VAABody, VAA},
};
use crate::governance::{BodyUpdateGuardianSet, BodyContractUpgrade};

/// SigInfo contains metadata about signers in a VerifySignature ix
struct SigInfo {
    /// index of the signer in the guardianset
    signer_index: u8,
    /// index of the signature in the secp instruction
    sig_index: u8,
}

struct SecpInstructionPart<'a> {
    address: &'a [u8],
    signature: &'a [u8],
    msg_offset: u16,
    msg_size: u16,
}

/// Instruction processing logic
impl Bridge {
    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = BridgeInstruction::deserialize(input)?;
        match instruction {
            Initialize(payload) => {
                msg!("Instruction: Initialize");
                Self::process_initialize(
                    program_id,
                    accounts,
                    payload.len_guardians,
                    payload.initial_guardian,
                    payload.config,
                )
            }
            PublishMessage(p) => {
                msg!("Instruction: PublishMessage");

                Self::process_publish_message(program_id, accounts, &p)
            }
            PostVAA(vaa_body) => {
                msg!("Instruction: PostVAA");
                let vaa = VAA::deserialize(&vaa_body)?;

                Self::process_vaa(program_id, accounts, vaa_body, &vaa)
            }
            VerifySignatures(p) => {
                msg!("Instruction: VerifySignatures");

                Self::process_verify_signatures(program_id, accounts, &p)
            }
        }
    }

    /// Unpacks a token state from a bytes buffer while assuring that the state is initialized.
    pub fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        len_guardians: u8,
        initial_guardian_key: [[u8; 20]; MAX_LEN_GUARDIAN_KEYS],
        config: BridgeConfig,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        next_account_info(account_info_iter)?; // System program
        let clock_info = next_account_info(account_info_iter)?;
        let new_bridge_info = next_account_info(account_info_iter)?;
        let new_guardian_info = next_account_info(account_info_iter)?;
        let payer_info = next_account_info(account_info_iter)?;

        let clock = Clock::from_account_info(clock_info)?;

        // Create bridge account
        let bridge_seed = Bridge::derive_bridge_seeds();
        // Will fail if the account already exists
        Bridge::check_and_create_account::<Bridge>(
            program_id,
            accounts,
            new_bridge_info.key,
            payer_info,
            program_id,
            &bridge_seed,
            None,
        )?;

        let mut new_account_data = new_bridge_info.try_borrow_mut_data()?;
        let mut bridge: &mut Bridge = Self::unpack(&mut new_account_data)?;

        // Create guardian set account
        let guardian_seed = Bridge::derive_guardian_set_seeds(new_bridge_info.key, 0);
        // Will fail if the account already exists
        Bridge::check_and_create_account::<GuardianSet>(
            program_id,
            accounts,
            new_guardian_info.key,
            payer_info,
            program_id,
            &guardian_seed,
            None,
        )?;

        let mut new_guardian_data = new_guardian_info.try_borrow_mut_data().map_err(|_| ProgramError::AccountBorrowFailed)?;
        let mut guardian_info: &mut GuardianSet = Self::unpack(&mut new_guardian_data)?;

        if len_guardians > MAX_LEN_GUARDIAN_KEYS as u8 {
            return Err(ProgramError::InvalidInstructionData);
        }

        // Initialize bridge params
        bridge.guardian_set_index = 0;
        bridge.config = config;

        // Initialize the initial guardian set
        guardian_info.index = 0;
        guardian_info.creation_time = clock.unix_timestamp.as_();
        guardian_info.keys = initial_guardian_key;
        guardian_info.len_keys = len_guardians;

        Ok(())
    }

    /// Processes signature verifications
    pub fn process_verify_signatures(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        payload: &VerifySigPayload,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        next_account_info(account_info_iter)?; // Bridge program
        next_account_info(account_info_iter)?; // System program
        let instruction_accounts = next_account_info(account_info_iter)?;
        let bridge_info = Self::next_account_info_with_owner(account_info_iter, program_id)?;
        let sig_info = next_account_info(account_info_iter)?;
        let guardian_set_info = Self::next_account_info_with_owner(account_info_iter, program_id)?;
        let payer_info = next_account_info(account_info_iter)?;

        if *instruction_accounts.key != solana_program::sysvar::instructions::id() {
            return Err(Error::InvalidSysvar.into());
        }

        // Verify bridge key because it is used as subsidizer
        let expected_bridge_key = Self::derive_bridge_id(program_id)?;
        if *bridge_info.key != expected_bridge_key {
            return Err(ProgramError::InvalidAccountData);
        }

        let guardian_data = guardian_set_info.try_borrow_data()?;
        let guardian_set: &GuardianSet = Self::unpack_immutable(&guardian_data)?;

        let sig_infos: Vec<SigInfo> = payload
            .signers
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if *p == -1 {
                    return None;
                }

                return Some(SigInfo {
                    sig_index: *p as u8,
                    signer_index: i as u8,
                });
            })
            .collect();

        let current_instruction = solana_program::sysvar::instructions::load_current_index(
            &instruction_accounts.try_borrow_mut_data()?,
        );
        if current_instruction == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        // The previous ix must be a secp verification instruction
        let secp_ix_index = (current_instruction - 1) as u8;
        let secp_ix = solana_program::sysvar::instructions::load_instruction_at(
            secp_ix_index as usize,
            &instruction_accounts.try_borrow_mut_data()?,
        )
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // Check that the instruction is actually for the secp program
        if secp_ix.program_id != solana_program::secp256k1_program::id() {
            return Err(ProgramError::InvalidArgument);
        }

        let secp_data_len = secp_ix.data.len();
        if secp_data_len < 2 {
            return Err(ProgramError::InvalidAccountData);
        }

        let sig_len = secp_ix.data[0];
        let mut index = 1;

        let mut secp_ixs: Vec<SecpInstructionPart> = Vec::with_capacity(sig_len as usize);
        for i in 0..sig_len {
            let sig_offset = byteorder::LE::read_u16(&secp_ix.data[index..index + 2]) as usize;
            index += 2;
            let sig_ix = secp_ix.data[index];
            index += 1;
            let address_offset = byteorder::LE::read_u16(&secp_ix.data[index..index + 2]) as usize;
            index += 2;
            let address_ix = secp_ix.data[index];
            index += 1;
            let msg_offset = byteorder::LE::read_u16(&secp_ix.data[index..index + 2]);
            index += 2;
            let msg_size = byteorder::LE::read_u16(&secp_ix.data[index..index + 2]);
            index += 2;
            let msg_ix = secp_ix.data[index];
            index += 1;

            if address_ix != secp_ix_index || msg_ix != secp_ix_index || sig_ix != secp_ix_index {
                return Err(ProgramError::InvalidArgument);
            }

            let address: &[u8] = &secp_ix.data[address_offset..address_offset + 20];
            let signature: &[u8] = &secp_ix.data[sig_offset..sig_offset + 65];

            // Make sure that all messages are equal
            if i > 0 {
                if msg_offset != secp_ixs[0].msg_offset || msg_size != secp_ixs[0].msg_size {
                    return Err(ProgramError::InvalidArgument);
                }
            }
            secp_ixs.push(SecpInstructionPart {
                address,
                signature,
                msg_offset,
                msg_size,
            });
        }

        if sig_infos.len() != secp_ixs.len() {
            return Err(ProgramError::InvalidArgument);
        }

        // Check message
        let message = &secp_ix.data[secp_ixs[0].msg_offset as usize
            ..(secp_ixs[0].msg_offset + secp_ixs[0].msg_size) as usize];

        let mut h = sha3::Keccak256::default();
        if let Err(_) = h.write(message) {
            return Err(ProgramError::InvalidArgument);
        };
        let msg_hash: [u8; 32] = h.finalize().into();
        if msg_hash != payload.hash {
            return Err(ProgramError::InvalidArgument);
        }

        // Track whether the account needs initialization
        let mut initialize_account = false;
        // Prepare message/payload-specific sig_info account
        if sig_info.data_is_empty() {
            let bridge_key = Bridge::derive_bridge_id(program_id)?;
            let sig_seeds =
                Bridge::derive_signature_seeds(&bridge_key, &msg_hash, guardian_set.index);
            Bridge::check_and_create_account::<SignatureState>(
                program_id,
                accounts,
                sig_info.key,
                payer_info,
                program_id,
                &sig_seeds,
                Some(bridge_info),
            )?;
            initialize_account = true;
        } else if payload.initial_creation {
            return Err(Error::AlreadyExists.into());
        }

        let mut sig_state_data = sig_info.try_borrow_mut_data()?;
        let mut sig_state: &mut SignatureState = Self::unpack(&mut sig_state_data)?;

        if initialize_account {
            sig_state.guardian_set_index = guardian_set.index;
            sig_state.hash = payload.hash;
        } else {
            // If the account already existed, check that the parameters match
            if sig_state.guardian_set_index != guardian_set.index {
                return Err(Error::GuardianSetMismatch.into());
            }
            if sig_state.hash != payload.hash {
                return Err(ProgramError::InvalidArgument);
            }
        }

        // Write sigs of checked addresses into sig_state
        for s in sig_infos {
            if s.signer_index > guardian_set.len_keys {
                return Err(ProgramError::InvalidArgument);
            }

            if s.sig_index + 1 > sig_len {
                return Err(ProgramError::InvalidArgument);
            }

            let key = guardian_set.keys[s.signer_index as usize];
            // Check key in ix
            if key != secp_ixs[s.sig_index as usize].address {
                return Err(ProgramError::InvalidArgument);
            }

            // Overwritten content should be zeros except double signs by the signer or harmless replays
            sig_state.signatures[s.signer_index as usize]
                .copy_from_slice(secp_ixs[s.sig_index as usize].signature);
        }

        Ok(())
    }

    /// Publish a message to the Wormhole protocol
    pub fn process_publish_message(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        t: &PublishMessagePayload,
    ) -> ProgramResult {
        msg!("publishing message");
        let account_info_iter = &mut accounts.iter();

        // Unused accounts.
        next_account_info(account_info_iter)?; // Bridge program
        next_account_info(account_info_iter)?; // System program
        next_account_info(account_info_iter)?; // Rent sysvar

        // solana_program::sysvar::clock::id() verified by Clock::from_account_info.
        let clock_info = next_account_info(account_info_iter)?;
        // solana_program::sysvar::instructions::id() verified below.
        let instructions_info = next_account_info(account_info_iter)?;
        // owner == bridge program asserted by Self::next_account_info_with_owner.
        let bridge_info = Self::next_account_info_with_owner(account_info_iter, program_id)?;
        // Derived account verified to match the expected pubkey via Bridge::check_and_create_account.
        let message_info = next_account_info(account_info_iter)?;
        // No need to verify - only used as the fee payer for account creation.
        let payer_info = next_account_info(account_info_iter)?;
        // The emitter, only used as metadata. We verify that the account is a signer
        // to prevent messages from being spoofed.
        let emitter_info = next_account_info(account_info_iter)?;

        let clock = Clock::from_account_info(clock_info)?;

        if *instructions_info.key != solana_program::sysvar::instructions::id() {
            return Err(Error::InvalidSysvar.into());
        }

        // The message emitter must be a signer
        if !emitter_info.is_signer {
            return Err(Error::EmitterNotSigner.into());
        }

        // Fee handling
        let fee = Self::transfer_fee();
        Self::check_fees(instructions_info, bridge_info, fee)?;

        // Create transfer account
        let message_seed = Bridge::derive_message_seeds(
            bridge_info.key,
            CHAIN_ID_SOLANA,
            emitter_info.key.to_bytes(),
            t.nonce,
            &t.payload,
        );
        Bridge::check_and_create_account::<PostedMessage>(
            program_id,
            accounts,
            message_info.key,
            payer_info,
            program_id,
            &message_seed,
            None,
        )?;

        let mut message_data = message_info.try_borrow_mut_data()?;
        let mut message: &mut PostedMessage = Self::unpack(&mut message_data)?;

        // Initialize transfer
        message.submission_time = clock.unix_timestamp as u32;
        message.emitter_chain = CHAIN_ID_SOLANA;
        message.emitter_address = emitter_info.key.to_bytes();
        message.nonce = t.nonce;
        message.payload = t.payload;

        Ok(())
    }

    /// Verify that a certain fee was sent to the bridge in the preceding instruction
    pub fn check_fees(
        instructions_info: &AccountInfo,
        bridge_info: &AccountInfo,
        fee: u64,
    ) -> Result<(), ProgramError> {
        let current_instruction = solana_program::sysvar::instructions::load_current_index(
            &instructions_info.try_borrow_mut_data()?,
        );
        if current_instruction == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        // The previous ix must be a transfer instruction
        let transfer_ix_index = (current_instruction - 1) as u8;
        let transfer_ix = solana_program::sysvar::instructions::load_instruction_at(
            transfer_ix_index as usize,
            &instructions_info.try_borrow_mut_data()?,
        )
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // Check that the instruction is actually for the system program
        if transfer_ix.program_id != solana_program::system_program::id() {
            return Err(ProgramError::InvalidArgument);
        }

        if transfer_ix.accounts.len() != 2 {
            return Err(ProgramError::InvalidInstructionData);
        }

        // Check that the fee was transferred to the bridge config.
        // We only care that the fee was sent to the bridge, not by whom it was sent.
        if transfer_ix.accounts[1].pubkey != *bridge_info.key {
            return Err(ProgramError::InvalidArgument);
        }

        // The transfer instruction is serialized using bincode (little endian)
        // uint32 ix_type = 2 (Transfer)
        // uint64 lamports
        // LEN: 4 + 8 = 12 bytes
        if transfer_ix.data.len() != 12 {
            return Err(ProgramError::InvalidAccountData);
        }

        // Verify action
        if transfer_ix.data[..4] != [2, 0, 0, 0] {
            return Err(ProgramError::InvalidInstructionData);
        }

        // Parse amount
        let mut fixed_data = [0u8; 8];
        fixed_data.copy_from_slice(&transfer_ix.data[4..]);
        let amount = u64::from_le_bytes(fixed_data);

        // Verify fee amount
        if amount < fee {
            return Err(Error::InsufficientFees.into());
        }

        Ok(())
    }

    pub fn transfer_sol(
        payer_account: &AccountInfo,
        recipient_account: &AccountInfo,
        amount: u64,
    ) -> ProgramResult {
        let mut payer_balance = payer_account.try_borrow_mut_lamports()?;
        **payer_balance = payer_balance
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;
        let mut recipient_balance = recipient_account.try_borrow_mut_lamports()?;
        **recipient_balance = recipient_balance
            .checked_add(amount)
            .ok_or(ProgramError::InvalidArgument)?;

        Ok(())
    }

    /// Processes a VAA
    pub fn process_vaa(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        vaa_data: VAAData,
        vaa: &VAA,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        // Load VAA processing default accounts
        next_account_info(account_info_iter)?; // Bridge program
        next_account_info(account_info_iter)?; // System program
        next_account_info(account_info_iter)?; // Rent sysvar
        let clock_info = next_account_info(account_info_iter)?;
        let bridge_info = Self::next_account_info_with_owner(account_info_iter, program_id)?;
        let guardian_set_info = Self::next_account_info_with_owner(account_info_iter, program_id)?;
        let claim_info = next_account_info(account_info_iter)?;
        let sig_info = Self::next_account_info_with_owner(account_info_iter, program_id)?;
        let payer_info = next_account_info(account_info_iter)?;

        let clock = Clock::from_account_info(clock_info)?;
        let mut guardian_data = guardian_set_info.try_borrow_mut_data()?;
        let guardian_set: &mut GuardianSet = Bridge::unpack(&mut guardian_data)?;

        // Verify bridge key because it is used as subsidizer and for key derivation
        let expected_bridge_key = Self::derive_bridge_id(program_id)?;
        if *bridge_info.key != expected_bridge_key {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check that the guardian set is valid
        let expected_guardian_set =
            Bridge::derive_guardian_set_id(program_id, bridge_info.key, vaa.guardian_set_index)?;
        if expected_guardian_set != *guardian_set_info.key {
            return Err(Error::InvalidDerivedAccount.into());
        }

        // Check that the guardian set is still active
        if guardian_set.expiration_time != 0
            && (guardian_set.expiration_time as i64) < clock.unix_timestamp
        {
            return Err(Error::GuardianSetExpired.into());
        }

        // Verify sig state
        let mut sig_state_data = sig_info.try_borrow_mut_data()?;
        let sig_state: &SignatureState = Self::unpack(&mut sig_state_data)?;

        // Verify that signatures were made using the correct set
        if sig_state.guardian_set_index != guardian_set.index {
            return Err(Error::GuardianSetMismatch.into());
        }

        let hash = vaa.body_hash()?;
        if sig_state.hash != hash {
            return Err(ProgramError::InvalidAccountData);
        }

        let signature_count = sig_state
            .signatures
            .iter()
            .filter(|v| v.iter().filter(|v| **v != 0).count() != 0)
            .count() as u8;
        // Check quorum
        // We're using a fixed point number transformation with 1 decimal to deal with rounding.
        // The cast to u16 exists to prevent issues where len_keys * 10 might overflow.
        if (signature_count as u16) < ((((guardian_set.len_keys as u16) * 10 / 3) * 2) / 10 + 1) {
            return Err(ProgramError::InvalidArgument);
        }


        // Process the message posting
        Self::process_vaa_message_post(
            program_id,
            account_info_iter,
            bridge_info,
            vaa,
            sig_info.key,
        )?;

        // Refund tx fee if possible
        if bridge_info
            .lamports()
            .checked_sub(Self::MIN_BRIDGE_BALANCE)
            .unwrap_or(0)
            >= Self::VAA_TX_FEE
        {
            Self::transfer_sol(bridge_info, payer_info, Self::VAA_TX_FEE)?;
        }

        // Load claim account
        let mut claim_data = claim_info.try_borrow_mut_data()?;
        let claim: &mut ClaimedVAA = Bridge::unpack(&mut claim_data)?;

        // Set claimed
        claim.vaa_time = clock.unix_timestamp as u32;

        Ok(())
    }

    /// Processes a Guardian set update
    pub fn process_governance_guardian_update(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        account_info_iter: &mut Iter<AccountInfo>,
        clock: &Clock,
        bridge_info: &AccountInfo,
        payer_info: &AccountInfo,
        bridge: &mut Bridge,
        old_guardian_set: &mut GuardianSet,
        b: &BodyUpdateGuardianSet,
    ) -> ProgramResult {
        let new_guardian_info = next_account_info(account_info_iter)?;

        // The new guardian set must be signed by the current one
        if bridge.guardian_set_index != old_guardian_set.index {
            return Err(Error::OldGuardianSet.into());
        }

        // The new guardian set must have an index > current
        // We don't check +1 because we trust the set to not set something close to max(u32)
        if bridge.guardian_set_index + 1 != b.new_index {
            return Err(Error::GuardianIndexNotIncreasing.into());
        }

        // Set the exirity on the old guardian set
        old_guardian_set.expiration_time =
            (clock.unix_timestamp as u32) + bridge.config.guardian_set_expiration_time;

        // Check whether the new guardian set was derived correctly
        let guardian_seed = Bridge::derive_guardian_set_seeds(bridge_info.key, b.new_index);
        // Will fail if the guardianset already exists
        Bridge::check_and_create_account::<GuardianSet>(
            program_id,
            accounts,
            new_guardian_info.key,
            payer_info,
            program_id,
            &guardian_seed,
            None,
        )?;

        let mut guardian_set_new_data = new_guardian_info.try_borrow_mut_data()?;
        let guardian_set_new: &mut GuardianSet =
            Bridge::unpack(&mut guardian_set_new_data)?;

        if b.new_keys.len() == 0 {
            return Err(Error::InvalidVAAFormat.into());
        }

        if b.new_keys.len() > MAX_LEN_GUARDIAN_KEYS {
            return Err(Error::InvalidVAAFormat.into());
        }

        // Force the new guardian set to not expire
        guardian_set_new.expiration_time = 0;
        guardian_set_new.index = b.new_index;
        let mut new_guardians = [[0u8; 20]; MAX_LEN_GUARDIAN_KEYS];
        for n in 0..b.new_keys.len() {
            new_guardians[n] = b.new_keys[n]
        }
        guardian_set_new.keys = new_guardians;
        guardian_set_new.len_keys = b.new_keys.len() as u8;
        guardian_set_new.creation_time = clock.unix_timestamp as u32;

        // Update the bridge guardian set id
        bridge.guardian_set_index = b.new_index;

        Ok(())
    }

    /// Processes a VAA post for data availability (for Solana -> foreign transfers)
    pub fn process_vaa_message_post(
        program_id: &Pubkey,
        account_info_iter: &mut Iter<AccountInfo>,
        bridge_info: &AccountInfo,
        vaa: &VAA,
        sig_account: &Pubkey,
    ) -> ProgramResult {
        msg!("posting VAA message");
        let message_account = Self::next_account_info_with_owner(account_info_iter, program_id)?;

        // Check whether the proposal was derived correctly
        let expected_message_address = Bridge::derive_message_id(
            program_id,
            bridge_info.key,
            b.emitter_chain,
            b.emitter_address,
            b.nonce,
            &b.data,
        )?;
        if expected_message_address != *message_account.key {
            return Err(Error::InvalidDerivedAccount.into());
        }

        let mut message_data = message_account.try_borrow_mut_data()?;
        let mut message: &mut PostedMessage = Self::unpack(&mut message_data)?;
        if !message.matches_vaa(b) {
            return Err(Error::VAAMessageMismatch.into());
        }
        if message.vaa_time != 0 {
            return Err(Error::VAAAlreadySubmitted.into());
        }

        message.vaa_version = vaa.version;
        message.vaa_time = vaa.timestamp;
        message.vaa_signature_account = *sig_account;

        Ok(())
    }

    /// Processes a VAA contract upgrade
    pub fn process_governance_upgrade(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bridge_info: &AccountInfo,
        b: &BodyContractUpgrade,
    ) -> ProgramResult {
        // Invoke upgrade
        let upgrade_ix = solana_program::bpf_loader_upgradeable::upgrade(
            program_id,
            &b.buffer,
            bridge_info.key,
            bridge_info.key,
        );
        Self::invoke_as_bridge(program_id, &upgrade_ix, accounts)?;

        Ok(())
    }
}

/// Implementation of actions
impl Bridge {
    pub fn invoke_as_bridge<'a>(
        program_id: &Pubkey,
        instruction: &Instruction,
        account_infos: &[AccountInfo<'a>],
    ) -> ProgramResult {
        let (_, seeds) =
            Self::find_program_address(&vec!["bridge".as_bytes().to_vec()], program_id);
        Self::invoke_vec_seed(program_id, instruction, account_infos, &seeds)
    }

    pub fn invoke_vec_seed<'a>(
        _program_id: &Pubkey,
        instruction: &Instruction,
        account_infos: &[AccountInfo<'a>],
        seeds: &Vec<Vec<u8>>,
    ) -> ProgramResult {
        let s: Vec<_> = seeds.iter().map(|item| item.as_slice()).collect();
        invoke_signed(instruction, account_infos, &[s.as_slice()])
    }

    /// The amount of sol that needs to be held in the BridgeConfig account in order to make it
    /// exempt of rent payments.
    const MIN_BRIDGE_BALANCE: u64 =
        (((solana_program::rent::ACCOUNT_STORAGE_OVERHEAD + size_of::<Bridge>() as u64)
            * solana_program::rent::DEFAULT_LAMPORTS_PER_BYTE_YEAR) as f64
            * solana_program::rent::DEFAULT_EXEMPTION_THRESHOLD) as u64;

    /// Check that a key was derived correctly and create account
    pub fn check_and_create_account<T: Sized>(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        new_account: &Pubkey,
        payer: &AccountInfo,
        owner: &Pubkey,
        seeds: &Vec<Vec<u8>>,
        subsidizer: Option<&AccountInfo>,
    ) -> Result<Vec<Vec<u8>>, ProgramError> {
        msg!("deriving key");
        let (expected_key, full_seeds) = Bridge::derive_key(program_id, seeds)?;
        if expected_key != *new_account {
            return Err(Error::InvalidDerivedAccount.into());
        }

        msg!("deploying contract");
        Self::create_account_raw::<T>(
            program_id,
            accounts,
            new_account,
            payer.key,
            owner,
            &full_seeds,
        )?;

        // The subsidizer refunds the rent that needs to be paid to create the account.
        // This mechanism is intended to reduce the cost of operating a guardian.
        // The subsidizer account should be of the type BridgeConfig and will only pay out
        // the subsidy if the account holds at least MIN_BRIDGE_BALANCE+rent
        match subsidizer {
            None => {}
            Some(v) => {
                let bal = v.try_lamports()?;
                let rent = Rent::default().minimum_balance(size_of::<T>());
                if bal
                    .checked_sub(Self::MIN_BRIDGE_BALANCE)
                    .ok_or(ProgramError::InsufficientFunds)?
                    >= rent
                {
                    // Refund rent to payer
                    Self::transfer_sol(v, payer, rent)?;
                }
            }
        }

        Ok(full_seeds)
    }

    /// Create a new account
    fn create_account_raw<T: Sized>(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        new_account: &Pubkey,
        payer: &Pubkey,
        owner: &Pubkey,
        seeds: &Vec<Vec<u8>>,
    ) -> Result<(), ProgramError> {
        let size = size_of::<T>();
        let ix = create_account(
            payer,
            new_account,
            Rent::default().minimum_balance(size as usize),
            size as u64,
            owner,
        );
        let s: Vec<_> = seeds.iter().map(|item| item.as_slice()).collect();
        invoke_signed(&ix, accounts, &[s.as_slice()])
    }

    /// Get the next account info from the iterator and check that it has the given owner
    pub fn next_account_info_with_owner<'a, 'b, I: Iterator<Item=&'a AccountInfo<'b>>>(
        iter: &mut I,
        owner: &Pubkey,
    ) -> Result<I::Item, ProgramError> {
        let acc = iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
        if acc.owner != owner {
            return Err(Error::InvalidOwner.into());
        }
        Ok(acc)
    }
}
