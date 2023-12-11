use flat_fee_interface::{FlatFeeError, ProgramState, SetLpWithdrawalFeeKeys};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{program::STATE_ID, utils::try_program_state};

pub struct SetLpWithdrawalFeeFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub state_acc: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> SetLpWithdrawalFeeFreeArgs<S> {
    pub fn resolve(self) -> Result<SetLpWithdrawalFeeKeys, FlatFeeError> {
        let SetLpWithdrawalFeeFreeArgs { state_acc } = self;

        if *state_acc.pubkey() != STATE_ID {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetLpWithdrawalFeeKeys {
            manager: state.manager,
            state: STATE_ID,
        })
    }
}
