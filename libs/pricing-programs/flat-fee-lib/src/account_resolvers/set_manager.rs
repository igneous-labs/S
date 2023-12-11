use flat_fee_interface::{FlatFeeError, ProgramState, SetManagerKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{program::STATE_ID, utils::try_program_state};

pub struct SetManagerFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub new_manager: Pubkey,
    pub state_acc: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> SetManagerFreeArgs<S> {
    pub fn resolve(self) -> Result<SetManagerKeys, FlatFeeError> {
        let SetManagerFreeArgs {
            new_manager,
            state_acc,
        } = self;

        if *state_acc.pubkey() != STATE_ID {
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
