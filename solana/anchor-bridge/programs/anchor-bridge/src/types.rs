use anchor_lang::prelude::*;

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct BridgeConfig {
    /// Period for how long a guardian set is valid after it has been replaced by a new one.  This
    /// guarantees that VAAs issued by that set can still be submitted for a certain period.  In
    /// this period we still trust the old guardian set.
    pub guardian_set_expiration_time: u32,
}

