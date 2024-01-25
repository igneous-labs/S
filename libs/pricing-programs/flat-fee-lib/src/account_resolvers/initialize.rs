use flat_fee_interface::InitializeKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::{pda::ProgramStateFindPdaArgs, program};

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

    pub fn resolve_for_prog(&self, program_id: Pubkey) -> InitializeKeys {
        InitializeKeys {
            payer: self.payer,
            state: ProgramStateFindPdaArgs { program_id }
                .get_program_state_address_and_bump_seed()
                .0,
            system_program: system_program::ID,
        }
    }
}
