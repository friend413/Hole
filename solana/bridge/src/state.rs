//! Bridge transition types

use std::io::Write;
use std::mem::size_of;
use std::slice::Iter;
use std::str;

use num_traits::AsPrimitive;
use sha3::Digest;
use solana_sdk::clock::Clock;
use solana_sdk::hash::hash;
#[cfg(not(target_arch = "bpf"))]
use solana_sdk::instruction::Instruction;
use solana_sdk::log::sol_log;
#[cfg(target_arch = "bpf")]
use solana_sdk::program::invoke_signed;
use solana_sdk::rent::Rent;
use solana_sdk::system_instruction::{create_account, SystemInstruction};
use solana_sdk::sysvar::Sysvar;
use solana_sdk::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, info,
    program_error::ProgramError, pubkey::bs58, pubkey::Pubkey,
};
use spl_token::state::Mint;

use crate::instruction::BridgeInstruction::*;
use crate::instruction::{
    BridgeInstruction, ForeignAddress, GuardianKey, TransferOutPayload, CHAIN_ID_SOLANA, VAA_BODY,
};
#[cfg(not(target_arch = "bpf"))]
use crate::processor::invoke_signed;

use crate::syscalls::{sol_verify_schnorr, RawKey, SchnorrifyInput};
use crate::vaa::{BodyTransfer, BodyUpdateGuardianSet, VAABody, VAA};
use crate::{error::Error, instruction::unpack};
use zerocopy::AsBytes;

/// fee rate as a ratio
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Fee {
    /// denominator of the fee ratio
    pub denominator: u64,
    /// numerator of the fee ratio
    pub numerator: u64,
}

/// guardian set
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GuardianSet {
    /// index of the set
    pub index: u32,
    /// public key of the threshold schnorr set
    pub pubkey: RawKey,
    /// creation time
    pub creation_time: u32,
    /// expiration time when VAAs issued by this set are no longer valid
    pub expiration_time: u32,

    /// Is `true` if this structure has been initialized.
    pub is_initialized: bool,
}

impl IsInitialized for GuardianSet {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

/// proposal to transfer tokens to a foreign chain
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TransferOutProposal {
    /// amount to transfer
    pub amount: u64,
    /// chain id to transfer to
    pub to_chain_id: u8,
    /// address on the foreign chain to transfer to
    pub foreign_address: ForeignAddress,
    /// asset that is being transferred
    pub asset: AssetMeta,
    /// vaa to unlock the tokens on the foreign chain
    pub vaa: VAA_BODY,
    /// time the vaa was submitted
    pub vaa_time: u32,

    /// Is `true` if this structure has been initialized.
    pub is_initialized: bool,
}

impl IsInitialized for TransferOutProposal {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl TransferOutProposal {
    pub fn matches_vaa(&self, b: &BodyTransfer) -> bool {
        return b.amount == self.amount
            && b.target_address == self.foreign_address
            && b.target_chain == self.to_chain_id
            && b.asset == self.asset;
    }
}

/// record of a claimed VAA
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClaimedVAA {
    /// hash of the vaa
    pub hash: [u8; 32],
    /// time the vaa was submitted
    pub vaa_time: u32,

    /// Is `true` if this structure has been initialized.
    pub is_initialized: bool,
}

impl IsInitialized for ClaimedVAA {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

/// Metadata about an asset
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AssetMeta {
    /// Address of the token
    pub address: ForeignAddress,

    /// Chain of the token
    pub chain: u8,
}

/// Config for a bridge.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BridgeConfig {
    /// Period for how long a VAA is valid. This is also the period after a valid VAA has been
    /// published to a `TransferOutProposal` or `ClaimedVAA` after which the account can be evicted.
    /// This exists to guarantee data availability and prevent replays.
    pub vaa_expiration_time: u32,

    /// Token program that is used for this bridge
    pub token_program: Pubkey,
}

/// Bridge state.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bridge {
    /// the currently active guardian set
    pub guardian_set_index: u32,

    /// read-only config parameters for a bridge instance.
    pub config: BridgeConfig,

    /// Is `true` if this structure has been initialized.
    pub is_initialized: bool,
}

impl IsInitialized for Bridge {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

/// Implementation of serialization functions
impl Bridge {
    /// Deserializes a spl_token `Account`.
    pub fn token_account_deserialize(
        info: &AccountInfo,
    ) -> Result<spl_token::state::Account, Error> {
        Ok(*spl_token::state::unpack(&mut info.data.borrow_mut())
            .map_err(|_| Error::ExpectedAccount)?)
    }

    /// Deserializes a spl_token `Mint`.
    pub fn mint_deserialize(info: &AccountInfo) -> Result<spl_token::state::Mint, Error> {
        Ok(*spl_token::state::unpack(&mut info.data.borrow_mut())
            .map_err(|_| Error::ExpectedToken)?)
    }

    /// Deserializes a `Bridge`.
    pub fn bridge_deserialize(info: &AccountInfo) -> Result<Bridge, Error> {
        Ok(*Bridge::unpack(&mut info.data.borrow_mut()).map_err(|_| Error::ExpectedBridge)?)
    }

    /// Deserializes a `GuardianSet`.
    pub fn guardian_set_deserialize(info: &AccountInfo) -> Result<GuardianSet, Error> {
        Ok(*Bridge::unpack(&mut info.data.borrow_mut()).map_err(|_| Error::ExpectedGuardianSet)?)
    }

    /// Deserializes a `TransferOutProposal`.
    pub fn transfer_out_proposal_deserialize(
        info: &AccountInfo,
    ) -> Result<TransferOutProposal, Error> {
        Ok(*Bridge::unpack(&mut info.data.borrow_mut())
            .map_err(|_| Error::ExpectedTransferOutProposal)?)
    }

    /// Unpacks a token state from a bytes buffer while assuring that the state is initialized.
    pub fn unpack<T: IsInitialized>(input: &mut [u8]) -> Result<&mut T, ProgramError> {
        let mut_ref: &mut T = Self::unpack_unchecked(input)?;
        if !mut_ref.is_initialized() {
            return Err(Error::UninitializedState.into());
        }
        Ok(mut_ref)
    }
    /// Unpacks a token state from a bytes buffer without checking that the state is initialized.
    pub fn unpack_unchecked<T: IsInitialized>(input: &mut [u8]) -> Result<&mut T, ProgramError> {
        if input.len() != size_of::<T>() {
            return Err(ProgramError::InvalidAccountData);
        }
        #[allow(clippy::cast_ptr_alignment)]
        Ok(unsafe { &mut *(&mut input[0] as *mut u8 as *mut T) })
    }
}

/// Implementation of actions
impl Bridge {
    /// Burn a wrapped asset from account
    pub fn wrapped_burn(
        accounts: &[AccountInfo],
        token_program_id: &Pubkey,
        authority: &Pubkey,
        token_account: &Pubkey,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let all_signers: Vec<&Pubkey> = accounts
            .iter()
            .filter_map(|item| if item.is_signer { Some(item.key) } else { None })
            .collect();
        let ix = spl_token::instruction::burn(
            token_program_id,
            token_account,
            authority,
            all_signers.as_slice(),
            amount,
        )?;
        invoke_signed(&ix, accounts, &[])
    }

    /// Mint a wrapped asset to account
    pub fn wrapped_mint_to(
        accounts: &[AccountInfo],
        token_program_id: &Pubkey,
        mint: &Pubkey,
        destination: &Pubkey,
        bridge: &Pubkey,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let ix = spl_token::instruction::mint_to(
            token_program_id,
            mint,
            destination,
            bridge,
            &[],
            amount,
        )?;
        invoke_signed(&ix, accounts, &[&[&bridge.to_bytes()[..32]][..]])
    }

    /// Transfer tokens from a caller
    pub fn token_transfer_caller(
        accounts: &[AccountInfo],
        token_program_id: &Pubkey,
        source: &Pubkey,
        destination: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let all_signers: Vec<&Pubkey> = accounts
            .iter()
            .filter_map(|item| if item.is_signer { Some(item.key) } else { None })
            .collect();
        let ix = spl_token::instruction::transfer(
            token_program_id,
            source,
            destination,
            authority,
            all_signers.as_slice(),
            amount,
        )?;
        invoke_signed(&ix, accounts, &[])
    }

    /// Transfer tokens from a custody account
    pub fn token_transfer_custody(
        accounts: &[AccountInfo],
        token_program_id: &Pubkey,
        bridge: &Pubkey,
        source: &Pubkey,
        destination: &Pubkey,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let ix = spl_token::instruction::transfer(
            token_program_id,
            source,
            destination,
            bridge,
            &[],
            amount,
        )?;
        invoke_signed(&ix, accounts, &[&[&bridge.to_bytes()[..32]][..]])
    }

    /// Create a new account
    pub fn create_custody_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        token_program: &Pubkey,
        bridge: &Pubkey,
        account: &Pubkey,
        mint: &Pubkey,
        payer: &Pubkey,
    ) -> Result<(), ProgramError> {
        Self::create_account::<Mint>(
            program_id,
            accounts,
            mint,
            payer,
            Self::derive_custody_seeds(bridge, mint),
        )?;
        let ix = spl_token::instruction::initialize_account(token_program, account, mint, bridge)?;
        invoke_signed(&ix, accounts, &[&[&bridge.to_bytes()[..32]][..]])
    }

    /// Create a mint for a wrapped asset
    pub fn create_wrapped_mint(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        token_program: &Pubkey,
        mint: &Pubkey,
        bridge: &Pubkey,
        payer: &Pubkey,
        asset: &AssetMeta,
    ) -> Result<(), ProgramError> {
        Self::create_account::<Mint>(
            program_id,
            accounts,
            mint,
            payer,
            Self::derive_wrapped_asset_seeds(bridge, asset.chain, asset.address),
        )?;
        let ix =
            spl_token::instruction::initialize_mint(token_program, mint, None, Some(bridge), 0, 8)?;
        invoke_signed(&ix, accounts, &[&[&bridge.to_bytes()[..32]][..]])
    }

    /// Create a new account
    pub fn create_account<T: Sized>(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        new_account: &Pubkey,
        payer: &Pubkey,
        seeds: Vec<Vec<u8>>,
    ) -> Result<(), ProgramError> {
        let size = size_of::<T>();
        let ix = create_account(
            payer,
            new_account,
            Rent::default().minimum_balance(size as usize),
            size as u64,
            program_id,
        );
        let s: Vec<_> = seeds.iter().map(|item| item.as_slice()).collect();
        invoke_signed(&ix, accounts, &[s.as_slice()])
    }
}

/// Implementation of derivations
impl Bridge {
    /// Calculates derived seeds for a guardian set
    pub fn derive_guardian_set_seeds(bridge_key: &Pubkey, guardian_set_index: u32) -> Vec<Vec<u8>> {
        vec![
            "guardian".as_bytes().to_vec(),
            bridge_key.to_bytes().to_vec(),
            guardian_set_index.as_bytes().to_vec(),
        ]
    }

    /// Calculates derived seeds for a wrapped asset
    pub fn derive_wrapped_asset_seeds(
        bridge_key: &Pubkey,
        asset_chain: u8,
        asset: ForeignAddress,
    ) -> Vec<Vec<u8>> {
        vec![
            "wrapped".as_bytes().to_vec(),
            bridge_key.to_bytes().to_vec(),
            asset_chain.as_bytes().to_vec(),
            asset.as_bytes().to_vec(),
        ]
    }

    /// Calculates derived seeds for a transfer out
    pub fn derive_transfer_id_seeds(
        bridge_key: &Pubkey,
        asset_chain: u8,
        asset: ForeignAddress,
        target_chain: u8,
        target_address: ForeignAddress,
        user: ForeignAddress,
        slot: u64,
    ) -> Vec<Vec<u8>> {
        vec![
            "transfer".as_bytes().to_vec(),
            bridge_key.to_bytes().to_vec(),
            asset_chain.as_bytes().to_vec(),
            asset.as_bytes().to_vec(),
            target_chain.as_bytes().to_vec(),
            target_address.as_bytes().to_vec(),
            user.as_bytes().to_vec(),
            slot.as_bytes().to_vec(),
        ]
    }

    /// Calculates derived seeds for a bridge
    pub fn derive_bridge_seeds(program_id: &Pubkey) -> Vec<Vec<u8>> {
        vec![program_id.to_bytes().to_vec()]
    }

    /// Calculates derived seeds for a custody account
    pub fn derive_custody_seeds<'a>(bridge: &Pubkey, mint: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            "custody".as_bytes().to_vec(),
            bridge.to_bytes().to_vec(),
            mint.to_bytes().to_vec(),
        ]
    }

    /// Calculates derived seeds for a claim
    pub fn derive_claim_seeds<'a>(bridge: &Pubkey, hash: &[u8; 32]) -> Vec<Vec<u8>> {
        vec![
            "claim".as_bytes().to_vec(),
            bridge.to_bytes().to_vec(),
            hash.as_bytes().to_vec(),
        ]
    }

    /// Calculates a derived address for this program
    pub fn derive_bridge_id(program_id: &Pubkey) -> Result<Pubkey, Error> {
        Self::derive_key(program_id, Self::derive_bridge_seeds(program_id))
    }

    /// Calculates a derived address for a custody account
    pub fn derive_custody_id(
        program_id: &Pubkey,
        bridge: &Pubkey,
        mint: &Pubkey,
    ) -> Result<Pubkey, Error> {
        Self::derive_key(program_id, Self::derive_custody_seeds(bridge, mint))
    }

    /// Calculates a derived address for a claim account
    pub fn derive_claim_id(
        program_id: &Pubkey,
        bridge: &Pubkey,
        hash: &[u8; 32],
    ) -> Result<Pubkey, Error> {
        Self::derive_key(program_id, Self::derive_claim_seeds(bridge, hash))
    }

    /// Calculates a derived address for this program
    pub fn derive_guardian_set_id(
        program_id: &Pubkey,
        bridge_key: &Pubkey,
        guardian_set_index: u32,
    ) -> Result<Pubkey, Error> {
        Self::derive_key(
            program_id,
            Self::derive_guardian_set_seeds(bridge_key, guardian_set_index),
        )
    }

    /// Calculates a derived seeds for a wrapped asset
    pub fn derive_wrapped_asset_id(
        program_id: &Pubkey,
        bridge_key: &Pubkey,
        asset_chain: u8,
        asset: ForeignAddress,
    ) -> Result<Pubkey, Error> {
        Self::derive_key(
            program_id,
            Self::derive_wrapped_asset_seeds(bridge_key, asset_chain, asset),
        )
    }

    /// Calculates a derived address for a transfer out
    pub fn derive_transfer_id(
        program_id: &Pubkey,
        bridge_key: &Pubkey,
        asset_chain: u8,
        asset: ForeignAddress,
        target_chain: u8,
        target_address: ForeignAddress,
        user: ForeignAddress,
        slot: u64,
    ) -> Result<Pubkey, Error> {
        Self::derive_key(
            program_id,
            Self::derive_transfer_id_seeds(
                bridge_key,
                asset_chain,
                asset,
                target_chain,
                target_address,
                user,
                slot,
            ),
        )
    }

    fn derive_key(program_id: &Pubkey, seeds: Vec<Vec<u8>>) -> Result<Pubkey, Error> {
        let s: Vec<_> = seeds.iter().map(|item| item.as_slice()).collect();
        Pubkey::create_program_address(s.as_slice(), program_id)
            .or(Err(Error::InvalidProgramAddress))
    }
}

/// Check is a token state is initialized
pub trait IsInitialized {
    /// Is initialized
    fn is_initialized(&self) -> bool;
}
