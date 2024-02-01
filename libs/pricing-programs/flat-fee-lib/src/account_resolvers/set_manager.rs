use flat_fee_interface::{FlatFeeError, ProgramState, SetManagerKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{pda::ProgramStateFindPdaArgs, program as flat_fee_program, utils::try_program_state};

pub struct SetManagerFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub new_manager: Pubkey,
    pub state_acc: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> SetManagerFreeArgs<S> {
    pub fn resolve(self) -> Result<SetManagerKeys, FlatFeeError> {
        self.resolve_inner(flat_fee_program::STATE_ID)
    }

    pub fn resolve_for_prog(self, program_id: Pubkey) -> Result<SetManagerKeys, FlatFeeError> {
        let state_id = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;

        self.resolve_inner(state_id)
    }

    fn resolve_inner(self, state_id: Pubkey) -> Result<SetManagerKeys, FlatFeeError> {
        let SetManagerFreeArgs {
            new_manager,
            state_acc,
        } = self;

        if *state_acc.pubkey() != state_id {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetManagerKeys {
            current_manager: state.manager,
            new_manager,
            state: state_id,
        })
    }
}
