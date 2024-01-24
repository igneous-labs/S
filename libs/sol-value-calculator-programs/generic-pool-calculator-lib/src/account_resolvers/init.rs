use generic_pool_calculator_interface::InitKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::{pda::CalculatorStateFindPdaArgs, GenericPoolSolValCalc};

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

    pub fn resolve_for_prog(self, program_id: Pubkey) -> InitKeys {
        InitKeys {
            payer: self.payer,
            state: CalculatorStateFindPdaArgs { program_id }
                .get_calculator_state_address_and_bump_seed()
                .0,
            system_program: system_program::ID,
        }
    }
}
