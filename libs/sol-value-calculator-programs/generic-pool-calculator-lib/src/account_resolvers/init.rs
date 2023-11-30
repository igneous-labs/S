use generic_pool_calculator_interface::InitKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::GenericPoolSolValCalc;

pub struct InitFreeArgs {
    pub payer: Pubkey,
}

impl InitFreeArgs {
    pub fn resolve<P: GenericPoolSolValCalc>(self) -> InitKeys {
        InitKeys {
            payer: self.payer,
            state: P::CALCULATOR_STATE_PDA,
            system_program: system_program::ID,
        }
    }
}
