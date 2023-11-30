use flat_fee_interface::InitializeKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::program;

pub struct InitializeFreeArgs {
    pub payer: Pubkey,
}

impl InitializeFreeArgs {
    pub fn resolve(&self) -> InitializeKeys {
        InitializeKeys {
            payer: self.payer,
            state: program::STATE_ID,
            system_program: system_program::ID,
        }
    }
}
