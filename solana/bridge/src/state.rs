//! Bridge transition types

use std::mem::size_of;

use primitive_types::U256;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
use zerocopy::AsBytes;

use crate::{
    error::Error,
    instruction::{ForeignAddress, MAX_LEN_GUARDIAN_KEYS, MAX_VAA_SIZE},
    vaa::BodyTransfer,
};
use solana_program::program_pack::Pack;
use solana_program::rent::Rent;

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
    /// number of keys stored
    pub len_keys: u8,
    /// public key hashes of the guardian set
    pub keys: [[u8; 20]; MAX_LEN_GUARDIAN_KEYS],
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
pub struct TransferOutProposal {
    /// amount to transfer
    pub amount: U256,
    /// chain id to transfer to
    pub to_chain_id: u8,
    /// address the transfer was initiated from
    pub source_address: ForeignAddress,
    /// address on the foreign chain to transfer to
    pub foreign_address: ForeignAddress,
    /// asset that is being transferred
    pub asset: AssetMeta,
    /// nonce of the transfer
    pub nonce: u32,
    /// vaa to unlock the tokens on the foreign chain
    /// it is +1 byte long to make space for the termination byte
    pub vaa: [u8; MAX_VAA_SIZE + 1],
    /// time the vaa was submitted
    pub vaa_time: u32,
    /// time the lockup was created
    pub lockup_time: u32,
    /// times the proposal has been poked
    pub poke_counter: u8,
    /// Account where signatures are stored
    pub signature_account: Pubkey,

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

/// metadata tracking for wrapped assets
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WrappedAssetMeta {
    /// chain id of the native chain of this asset
    pub chain: u8,
    /// address of the asset on the native chain
    pub address: ForeignAddress,

    /// Is `true` if this structure has been initialized.
    pub is_initialized: bool,
}

impl IsInitialized for WrappedAssetMeta {
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

    /// Number of decimals of the token
    pub decimals: u8,
}

/// Config for a bridge.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BridgeConfig {
    /// Period for how long a guardian set is valid after it has been replaced by a new one.
    /// This guarantees that VAAs issued by that set can still be submitted for a certain period.
    /// In this period we still trust the old guardian set.
    pub guardian_set_expiration_time: u32,

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

/// Signature state
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SignatureState {
    /// signatures of validators
    pub signatures: [[u8; 65]; MAX_LEN_GUARDIAN_KEYS],

    /// hash of the data
    pub hash: [u8; 32],

    /// index of the guardian set
    pub guardian_set_index: u32,

    /// Is `true` if this structure has been initialized.
    pub is_initialized: bool,
}

impl IsInitialized for SignatureState {
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
        Ok(spl_token::state::Account::unpack(&mut info.data.borrow_mut())
            .map_err(|_| Error::ExpectedAccount)?)
    }

    /// Deserializes a spl_token `Mint`.
    pub fn mint_deserialize(info: &AccountInfo) -> Result<spl_token::state::Mint, Error> {
        Ok(spl_token::state::Mint::unpack(&mut info.data.borrow_mut())
            .map_err(|_| Error::ExpectedToken)?)
    }

    /// Unpacks a state from a bytes buffer while assuring that the state is initialized.
    pub fn unpack<T: IsInitialized>(input: &mut [u8]) -> Result<&mut T, ProgramError> {
        let mut_ref: &mut T = Self::unpack_unchecked(input)?;
        if !mut_ref.is_initialized() {
            return Err(Error::UninitializedState.into());
        }
        Ok(mut_ref)
    }

    /// Unpacks a state from a bytes buffer without checking that the state is initialized.
    pub fn unpack_unchecked<T: IsInitialized>(input: &mut [u8]) -> Result<&mut T, ProgramError> {
        if input.len() != size_of::<T>() {
            return Err(ProgramError::InvalidAccountData);
        }
        #[allow(clippy::cast_ptr_alignment)]
            Ok(unsafe { &mut *(&mut input[0] as *mut u8 as *mut T) })
    }

    /// Unpacks a state from a bytes buffer while assuring that the state is initialized.
    pub fn unpack_immutable<T: IsInitialized>(input: &[u8]) -> Result<&T, ProgramError> {
        let mut_ref: &T = Self::unpack_unchecked_immutable(input)?;
        if !mut_ref.is_initialized() {
            return Err(Error::UninitializedState.into());
        }
        Ok(mut_ref)
    }

    /// Unpacks a state from a bytes buffer without checking that the state is initialized.
    pub fn unpack_unchecked_immutable<T: IsInitialized>(input: &[u8]) -> Result<&T, ProgramError> {
        if input.len() != size_of::<T>() {
            return Err(ProgramError::InvalidAccountData);
        }
        #[allow(clippy::cast_ptr_alignment)]
            Ok(unsafe { &*(&input[0] as *const u8 as *const T) })
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
        asset_decimal: u8,
        asset: ForeignAddress,
    ) -> Vec<Vec<u8>> {
        vec![
            "wrapped".as_bytes().to_vec(),
            bridge_key.to_bytes().to_vec(),
            asset_chain.as_bytes().to_vec(),
            asset_decimal.as_bytes().to_vec(),
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
        sender: ForeignAddress,
        nonce: u32,
    ) -> Vec<Vec<u8>> {
        vec![
            "transfer".as_bytes().to_vec(),
            bridge_key.to_bytes().to_vec(),
            asset_chain.as_bytes().to_vec(),
            asset.as_bytes().to_vec(),
            target_chain.as_bytes().to_vec(),
            target_address.as_bytes().to_vec(),
            sender.as_bytes().to_vec(),
            nonce.as_bytes().to_vec(),
        ]
    }

    /// Calculates derived seeds for a bridge
    pub fn derive_bridge_seeds() -> Vec<Vec<u8>> {
        vec!["bridge".as_bytes().to_vec()]
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
    pub fn derive_claim_seeds<'a>(bridge: &Pubkey, body: Vec<u8>) -> Vec<Vec<u8>> {
        [
            vec!["claim".as_bytes().to_vec(), bridge.to_bytes().to_vec()],
            body.chunks(32).map(|v| v.to_vec()).collect(),
        ]
            .concat()
    }

    /// Calculates derived seeds for a wrapped asset meta entry
    pub fn derive_wrapped_meta_seeds<'a>(bridge: &Pubkey, mint: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            "meta".as_bytes().to_vec(),
            bridge.to_bytes().to_vec(),
            mint.to_bytes().to_vec(),
        ]
    }

    /// Calculates derived seeds for a signature account
    pub fn derive_signature_seeds<'a>(
        bridge: &Pubkey,
        hash: &[u8; 32],
        guardian_index: u32,
    ) -> Vec<Vec<u8>> {
        vec![
            "sig".as_bytes().to_vec(),
            bridge.to_bytes().to_vec(),
            hash.to_vec(),
            guardian_index.to_le_bytes().to_vec(),
        ]
    }

    /// Calculates a derived address for this program
    pub fn derive_bridge_id(program_id: &Pubkey) -> Result<Pubkey, Error> {
        Ok(Self::derive_key(program_id, &Self::derive_bridge_seeds())?.0)
    }

    /// Calculates a derived address for a custody account
    pub fn derive_custody_id(
        program_id: &Pubkey,
        bridge: &Pubkey,
        mint: &Pubkey,
    ) -> Result<Pubkey, Error> {
        Ok(Self::derive_key(program_id, &Self::derive_custody_seeds(bridge, mint))?.0)
    }

    /// Calculates a derived address for a claim account
    pub fn derive_claim_id(
        program_id: &Pubkey,
        bridge: &Pubkey,
        body: Vec<u8>,
    ) -> Result<Pubkey, Error> {
        Ok(Self::derive_key(program_id, &Self::derive_claim_seeds(bridge, body))?.0)
    }

    /// Calculates a derived address for a wrapped asset meta entry
    pub fn derive_wrapped_meta_id(
        program_id: &Pubkey,
        bridge: &Pubkey,
        mint: &Pubkey,
    ) -> Result<Pubkey, Error> {
        Ok(Self::derive_key(program_id, &Self::derive_wrapped_meta_seeds(bridge, mint))?.0)
    }

    /// Calculates a derived address for this program
    pub fn derive_guardian_set_id(
        program_id: &Pubkey,
        bridge_key: &Pubkey,
        guardian_set_index: u32,
    ) -> Result<Pubkey, Error> {
        Ok(Self::derive_key(
            program_id,
            &Self::derive_guardian_set_seeds(bridge_key, guardian_set_index),
        )?
            .0)
    }

    /// Calculates a derived seeds for a wrapped asset
    pub fn derive_wrapped_asset_id(
        program_id: &Pubkey,
        bridge_key: &Pubkey,
        asset_chain: u8,
        asset_decimal: u8,
        asset: ForeignAddress,
    ) -> Result<Pubkey, Error> {
        Ok(Self::derive_key(
            program_id,
            &Self::derive_wrapped_asset_seeds(bridge_key, asset_chain, asset_decimal, asset),
        )?
            .0)
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
        slot: u32,
    ) -> Result<Pubkey, Error> {
        Ok(Self::derive_key(
            program_id,
            &Self::derive_transfer_id_seeds(
                bridge_key,
                asset_chain,
                asset,
                target_chain,
                target_address,
                user,
                slot,
            ),
        )?
            .0)
    }

    /// Calculates derived address for a signature account
    pub fn derive_signature_id<'a>(
        program_id: &Pubkey,
        bridge: &Pubkey,
        hash: &[u8; 32],
        guardian_index: u32,
    ) -> Result<Pubkey, Error> {
        Ok(Self::derive_key(
            program_id,
            &Self::derive_signature_seeds(bridge, hash, guardian_index),
        )?
            .0)
    }

    pub fn derive_key(
        program_id: &Pubkey,
        seeds: &Vec<Vec<u8>>,
    ) -> Result<(Pubkey, Vec<Vec<u8>>), Error> {
        Ok(Self::find_program_address(seeds, program_id))
    }

    pub fn find_program_address(
        seeds: &Vec<Vec<u8>>,
        program_id: &Pubkey,
    ) -> (Pubkey, Vec<Vec<u8>>) {
        let mut nonce = [255u8];
        for _ in 0..std::u8::MAX {
            {
                let mut seeds_with_nonce = seeds.to_vec();
                seeds_with_nonce.push(nonce.to_vec());
                let s: Vec<_> = seeds_with_nonce
                    .iter()
                    .map(|item| item.as_slice())
                    .collect();
                if let Ok(address) = Pubkey::create_program_address(&s, program_id) {
                    return (address, seeds_with_nonce);
                }
            }
            nonce[0] -= 1;
        }
        panic!("Unable to find a viable program address nonce");
    }

    /// Tx fee of Signature checks and PostVAA (see docs for calculation)
    pub const VAA_TX_FEE: u64 = 18 * 10000;

    pub fn transfer_fee() -> u64 {
        // Pay for 2 signature state and Claimed VAA rents + 2 * guardian tx fees
        // This will pay for this transfer and ~10 inbound ones
        Rent::default().minimum_balance((size_of::<SignatureState>() + size_of::<ClaimedVAA>()) * 2) + Self::VAA_TX_FEE * 2
    }
}

/// Check is a token state is initialized
pub trait IsInitialized {
    /// Is initialized
    fn is_initialized(&self) -> bool;
}
