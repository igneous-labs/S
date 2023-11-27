use generic_pool_calculator_interface::InitKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::GenericPoolSolValCalc;

pub struct InitRootAccounts {
    pub payer: Pubkey,
}

impl InitRootAccounts {
    pub fn resolve<P: GenericPoolSolValCalc>(self) -> InitKeys {
        InitKeys {
            payer: self.payer,
            state: P::CALCULATOR_STATE_PDA,
            system_program: system_program::ID,
        }
    }
}
