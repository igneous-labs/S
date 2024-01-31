use flat_fee_interface::InitializeKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::{pda::ProgramStateFindPdaArgs, program::STATE_ID};

pub struct InitializeFreeArgs {
    pub payer: Pubkey,
}

impl InitializeFreeArgs {
    pub fn resolve(&self) -> InitializeKeys {
        self.resolve_inner(STATE_ID)
    }

    pub fn resolve_for_prog(&self, program_id: Pubkey) -> InitializeKeys {
        let state_id = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;
        self.resolve_inner(state_id)
    }

    fn resolve_inner(&self, state_id: Pubkey) -> InitializeKeys {
        InitializeKeys {
            payer: self.payer,
            state: state_id,
            system_program: system_program::ID,
        }
    }
}
