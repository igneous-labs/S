use flat_fee_interface::{FlatFeeError, ProgramState, SetManagerKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{program::STATE_ID, utils::try_program_state};

pub struct SetManagerFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub new_manager: Pubkey,
    pub state_acc: S,
}

impl<S: KeyedAccount + ReadonlyAccountData> SetManagerFreeArgs<S> {
    pub fn resolve(self) -> Result<SetManagerKeys, FlatFeeError> {
        let SetManagerFreeArgs {
            new_manager,
            state_acc,
        } = self;

        if *state_acc.key() != STATE_ID {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetManagerKeys {
            current_manager: state.manager,
            new_manager,
            state: STATE_ID,
        })
    }
}
