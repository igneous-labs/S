use flat_fee_interface::{FlatFeeError, ProgramState, SetLpWithdrawalFeeKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{pda::ProgramStateFindPdaArgs, program::STATE_ID, utils::try_program_state};

pub struct SetLpWithdrawalFeeFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub state_acc: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> SetLpWithdrawalFeeFreeArgs<S> {
    pub fn resolve(self) -> Result<SetLpWithdrawalFeeKeys, FlatFeeError> {
        self.resolve_inner(STATE_ID)
    }

    pub fn resolve_for_prog(
        self,
        program_id: Pubkey,
    ) -> Result<SetLpWithdrawalFeeKeys, FlatFeeError> {
        let state_id = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;
        self.resolve_inner(state_id)
    }

    fn resolve_inner(self, state_id: Pubkey) -> Result<SetLpWithdrawalFeeKeys, FlatFeeError> {
        let SetLpWithdrawalFeeFreeArgs { state_acc } = self;

        if *state_acc.pubkey() != state_id {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetLpWithdrawalFeeKeys {
            manager: state.manager,
            state: state_id,
        })
    }
}
