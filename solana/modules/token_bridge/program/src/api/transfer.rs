use crate::{
    accounts::{
        AuthoritySigner,
        ConfigAccount,
        CustodyAccount,
        CustodyAccountDerivationData,
        CustodySigner,
        EmitterAccount,
        MintSigner,
        WrappedDerivationData,
        WrappedMint,
        WrappedTokenMeta,
    },
    messages::PayloadTransfer,
    types::*,
    TokenBridgeError,
    TokenBridgeError::WrongAccountOwner,
};
use bridge::{
    api::{
        PostMessage,
        PostMessageData,
    },
    vaa::SerializePayload,
};
use primitive_types::U256;
use solana_program::{
    account_info::AccountInfo,
    instruction::{
        AccountMeta,
        Instruction,
    },
    program::{
        invoke,
        invoke_signed,
    },
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
    sysvar::clock::Clock,
};
use solitaire::{
    processors::seeded::{
        invoke_seeded,
        Seeded,
    },
    CreationLamports::Exempt,
    *,
};
use spl_token::{
    error::TokenError::OwnerMismatch,
    state::{
        Account,
        Mint,
    },
};
use std::ops::{
    Deref,
    DerefMut,
};

#[derive(FromAccounts)]
pub struct TransferNative<'b> {
    pub payer: Signer<AccountInfo<'b>>,
    pub config: ConfigAccount<'b, { AccountState::Initialized }>,

    pub from: Data<'b, SplAccount, { AccountState::Initialized }>,
    pub mint: Data<'b, SplMint, { AccountState::Initialized }>,

    pub custody: CustodyAccount<'b, { AccountState::MaybeInitialized }>,

    // This could allow someone to race someone else's tx if they do the approval in a separate tx.
    // Therefore the approval must be set in the same tx
    pub authority_signer: AuthoritySigner<'b>,
    pub custody_signer: CustodySigner<'b>,

    /// CPI Context
    pub bridge: Info<'b>,

    /// Account to store the posted message
    pub message: Info<'b>,

    /// Emitter of the VAA
    pub emitter: EmitterAccount<'b>,

    /// Tracker for the emitter sequence
    pub sequence: Info<'b>,

    /// Account to collect tx fee
    pub fee_collector: Info<'b>,

    pub clock: Sysvar<'b, Clock>,
}

impl<'a> From<&TransferNative<'a>> for CustodyAccountDerivationData {
    fn from(accs: &TransferNative<'a>) -> Self {
        CustodyAccountDerivationData {
            mint: *accs.mint.info().key,
        }
    }
}

impl<'b> InstructionContext<'b> for TransferNative<'b> {
    fn verify(&self, program_id: &Pubkey) -> Result<()> {
        // Verify that the custody account is derived correctly
        self.custody.verify_derivation(program_id, &self.into())?;

        // Verify mints
        if self.mint.info().key != self.from.info().key {
            return Err(TokenBridgeError::InvalidMint.into());
        }

        // Verify that the token is not a wrapped token
        if let COption::Some(mint_authority) = self.mint.mint_authority {
            if mint_authority == MintSigner::key(None, program_id) {
                return Err(TokenBridgeError::TokenNotNative.into());
            }
        }

        Ok(())
    }
}

#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct TransferNativeData {
    pub nonce: u32,
    pub amount: u64,
    pub fee: u64,
    pub target_address: Address,
    pub target_chain: ChainID,
}

pub fn transfer_native(
    ctx: &ExecutionContext,
    accs: &mut TransferNative,
    data: TransferNativeData,
) -> Result<()> {
    if !accs.custody.is_initialized() {
        accs.custody
            .create(&(&*accs).into(), ctx, accs.payer.key, Exempt)?;

        let init_ix = spl_token::instruction::initialize_account(
            &spl_token::id(),
            accs.custody.info().key,
            accs.mint.info().key,
            accs.custody_signer.key,
        )?;
        invoke_signed(&init_ix, ctx.accounts, &[])?;
    }

    // Transfer tokens
    let transfer_ix = spl_token::instruction::transfer(
        &spl_token::id(),
        accs.from.info().key,
        accs.custody.info().key,
        accs.authority_signer.key,
        &[],
        data.amount,
    )?;
    invoke_seeded(&transfer_ix, ctx, &accs.authority_signer, None)?;

    // Pay fee
    let transfer_ix =
        solana_program::system_instruction::transfer(accs.payer.key, accs.fee_collector.key, 1000);
    invoke(&transfer_ix, ctx.accounts)?;

    // Post message
    let payload = PayloadTransfer {
        amount: U256::from(data.amount),
        token_address: accs.mint.info().key.to_bytes(),
        token_chain: 1,
        to: data.target_address,
        to_chain: data.target_chain,
        fee: U256::from(data.fee),
    };
    let params = bridge::instruction::Instruction::PostMessage(PostMessageData {
        nonce: data.nonce,
        payload: payload.try_to_vec()?,
    });

    let ix = Instruction::new_with_bytes(
        accs.config.wormhole_bridge,
        params.try_to_vec()?.as_slice(),
        vec![
            AccountMeta::new_readonly(*accs.bridge.key, false),
            AccountMeta::new(*accs.message.key, false),
            AccountMeta::new_readonly(*accs.emitter.key, true),
            AccountMeta::new(*accs.sequence.key, false),
            AccountMeta::new(*accs.payer.key, true),
            AccountMeta::new(*accs.fee_collector.key, false),
            AccountMeta::new_readonly(*accs.clock.info().key, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
        ],
    );
    invoke_seeded(&ix, ctx, &accs.emitter, None)?;

    Ok(())
}

#[derive(FromAccounts)]
pub struct TransferWrapped<'b> {
    pub payer: Signer<AccountInfo<'b>>,
    pub config: ConfigAccount<'b, { AccountState::Initialized }>,

    pub from: Data<'b, SplAccount, { AccountState::Initialized }>,
    pub from_owner: Signer<Info<'b>>,
    pub mint: WrappedMint<'b, { AccountState::Initialized }>,
    pub wrapped_meta: WrappedTokenMeta<'b, { AccountState::Initialized }>,

    pub mint_authority: MintSigner<'b>,

    /// CPI Context
    pub bridge: Info<'b>,

    /// Account to store the posted message
    pub message: Info<'b>,

    /// Emitter of the VAA
    pub emitter: EmitterAccount<'b>,

    /// Tracker for the emitter sequence
    pub sequence: Info<'b>,

    /// Account to collect tx fee
    pub fee_collector: Info<'b>,

    pub clock: Sysvar<'b, Clock>,
}

impl<'a> From<&TransferWrapped<'a>> for WrappedDerivationData {
    fn from(accs: &TransferWrapped<'a>) -> Self {
        WrappedDerivationData {
            token_chain: 1,
            token_address: accs.mint.info().key.to_bytes(),
        }
    }
}

impl<'b> InstructionContext<'b> for TransferWrapped<'b> {
    fn verify(&self, program_id: &Pubkey) -> Result<()> {
        // Verify that the from account is owned by the from_owner
        if &self.from.owner != self.from_owner.key {
            return Err(WrongAccountOwner.into());
        }

        // Verify mints
        if self.mint.info().key != &self.from.mint {
            return Err(TokenBridgeError::InvalidMint.into());
        }

        // Verify that meta is correct
        self.wrapped_meta
            .verify_derivation(program_id, &self.into())?;

        Ok(())
    }
}

#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct TransferWrappedData {
    pub nonce: u32,
    pub amount: u64,
    pub fee: u64,
    pub target_address: Address,
    pub target_chain: ChainID,
}

pub fn transfer_wrapped(
    ctx: &ExecutionContext,
    accs: &mut TransferWrapped,
    data: TransferWrappedData,
) -> Result<()> {
    // Burn tokens
    let burn_ix = spl_token::instruction::burn(
        &spl_token::id(),
        accs.from.info().key,
        accs.mint.info().key,
        accs.mint_authority.key,
        &[],
        data.amount,
    )?;
    invoke_seeded(&burn_ix, ctx, &accs.mint_authority, None)?;

    // Pay fee
    let transfer_ix =
        solana_program::system_instruction::transfer(accs.payer.key, accs.fee_collector.key, 1000);
    invoke(&transfer_ix, ctx.accounts)?;

    // Post message
    let payload = PayloadTransfer {
        amount: U256::from(data.amount),
        token_address: accs.wrapped_meta.token_address,
        token_chain: accs.wrapped_meta.chain,
        to: data.target_address,
        to_chain: data.target_chain,
        fee: U256::from(data.fee),
    };
    let params = bridge::instruction::Instruction::PostMessage(PostMessageData {
        nonce: data.nonce,
        payload: payload.try_to_vec()?,
    });

    let ix = Instruction::new_with_bytes(
        accs.config.wormhole_bridge,
        params.try_to_vec()?.as_slice(),
        vec![
            AccountMeta::new_readonly(*accs.bridge.key, false),
            AccountMeta::new(*accs.message.key, false),
            AccountMeta::new_readonly(*accs.emitter.key, true),
            AccountMeta::new(*accs.sequence.key, false),
            AccountMeta::new(*accs.payer.key, true),
            AccountMeta::new(*accs.fee_collector.key, false),
            AccountMeta::new_readonly(*accs.clock.info().key, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
        ],
    );
    invoke_seeded(&ix, ctx, &accs.emitter, None)?;

    Ok(())
}
