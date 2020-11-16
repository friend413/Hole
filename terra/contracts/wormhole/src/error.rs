use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    /// Invalid VAA version
    #[error("InvalidVersion")]
    InvalidVersion,

    /// Guardian set with this index does not exist
    #[error("InvalidGuardianSetIndex")]
    InvalidGuardianSetIndex,

    /// Guardian set expiration date is zero or in the past
    #[error("GuardianSetExpired")]
    GuardianSetExpired,

    /// Not enough signers on the VAA
    #[error("NoQuorum")]
    NoQuorum,

    /// Wrong guardian index order, order must be ascending
    #[error("WrongGuardianIndexOrder")]
    WrongGuardianIndexOrder,

    /// Some problem with signature decoding from bytes
    #[error("CannotDecodeSignature")]
    CannotDecodeSignature,

    /// Some problem with public key recovery from the signature
    #[error("CannotRecoverKey")]
    CannotRecoverKey,

    /// Recovered pubkey from signature does not match guardian address
    #[error("GuardianSignatureError")]
    GuardianSignatureError,

    /// VAA action code not recognized
    #[error("InvalidVAAAction")]
    InvalidVAAAction,

    /// VAA guardian set is not current
    #[error("NotCurrentGuardianSet")]
    NotCurrentGuardianSet,

    /// Only 128-bit amounts are supported
    #[error("AmountTooHigh")]
    AmountTooHigh,

    /// Amount should be higher than zero
    #[error("AmountTooLow")]
    AmountTooLow,

    /// Source and target chain ids must be different
    #[error("SameSourceAndTarget")]
    SameSourceAndTarget,

    /// Target chain id must be the same as the current CHAIN_ID
    #[error("WrongTargetChain")]
    WrongTargetChain,

    /// Wrapped asset init hook sent twice for the same asset id
    #[error("AssetAlreadyRegistered")]
    AssetAlreadyRegistered,

    /// Guardian set must increase in steps of 1
    #[error("GuardianSetIndexIncreaseError")]
    GuardianSetIndexIncreaseError,

    /// VAA was already executed
    #[error("VaaAlreadyExecuted")]
    VaaAlreadyExecuted,

    /// Message sender not permitted to execute this operation
    #[error("PermissionDenied")]
    PermissionDenied,

    /// Attempt to execute contract action while it is inactive
    #[error("ContractInactive")]
    ContractInactive,
}

impl ContractError {
    pub fn std(&self) -> StdError {
        StdError::GenericErr {
            msg: format!("{}", self),
            backtrace: None,
        }
    }

    pub fn std_err<T>(&self) -> Result<T, StdError> {
        Err(self.std())
    }
}
