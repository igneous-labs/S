use flat_fee_interface::InitKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::program;

pub struct InitFreeArgs {
    pub payer: Pubkey,
}

impl InitFreeArgs {
    pub fn resolve(&self) -> InitKeys {
        InitKeys {
            payer: self.payer,
            state: program::STATE_ID,
            system_program: system_program::ID,
        }
    }
}
