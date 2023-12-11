use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountOwner, ReadonlyAccountPubkey};

/// A mint and its owner token program
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct MintWithTokenProgram {
    /// The mint's pubkey
    pub pubkey: Pubkey,

    /// The mint's owner token program
    pub token_program: Pubkey,
}

impl ReadonlyAccountOwner for MintWithTokenProgram {
    fn owner(&self) -> &Pubkey {
        &self.token_program
    }
}

impl ReadonlyAccountPubkey for MintWithTokenProgram {
    fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }
}
