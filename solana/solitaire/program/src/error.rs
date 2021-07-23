use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Quality of life Result type for the Solitaire stack.
pub type Result<T> = std::result::Result<T, SolitaireError>;

/// Quality of life type alias for wrapping up boxed errors.
pub type ErrBox = Box<dyn std::error::Error>;

/// There are several places in Solitaire that might fail, we want descriptive errors.
#[derive(Debug)]
pub enum SolitaireError {
    /// The AccountInfo parser expected a mutable key where a readonly was found, or vice versa.
    InvalidMutability(Pubkey),

    /// The AccountInfo parser expected a Signer, but the account did not sign.
    InvalidSigner(Pubkey),

    /// The AccountInfo parser expected a Sysvar, but the key was invalid.
    InvalidSysvar(Pubkey),

    /// The AccountInfo parser tried to derive the provided key, but it did not match.
    InvalidDerive(Pubkey, Pubkey),

    /// The AccountInfo has an invalid owner.
    InvalidOwner(Pubkey),

    /// The AccountInfo is non-writeable where a writeable key was expected.
    NonWriteableAccount(Pubkey),

    /// The instruction payload itself could not be deserialized.
    InstructionDeserializeFailed(std::io::Error),

    /// An IO error was captured, wrap it up and forward it along.
    IoError(std::io::Error),

    /// An solana program error
    ProgramError(ProgramError),

    /// Owner of the account is ambiguous
    AmbiguousOwner,

    /// Account has already been initialized
    AlreadyInitialized(Pubkey),

    /// An instruction that wasn't recognised was sent.
    UnknownInstruction,

    Custom(u64),
}

impl From<ProgramError> for SolitaireError {
    fn from(e: ProgramError) -> Self {
        SolitaireError::ProgramError(e)
    }
}

impl From<std::io::Error> for SolitaireError {
    fn from(e: std::io::Error) -> Self {
        SolitaireError::IoError(e)
    }
}

impl Into<ProgramError> for SolitaireError {
    fn into(self) -> ProgramError {
        match self {
            SolitaireError::ProgramError(e) => return e,
            _ => ProgramError::Custom(0),
        }
    }
}
