use flat_fee_interface::{FlatFeeError, ProgramState, RemoveLstKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{
    pda::{FeeAccountFindPdaArgs, ProgramStateFindPdaArgs},
    program::{self, STATE_ID},
    utils::try_program_state,
};

pub struct RemoveLstFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub refund_rent_to: Pubkey,
    pub lst_mint: Pubkey,
    pub state_acc: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> RemoveLstFreeArgs<S> {
    /// Uses find_program_address().
    /// Ok to be inefficient since this is admin-facing
    pub fn resolve(self) -> Result<RemoveLstKeys, FlatFeeError> {
        self.resolve_inner(STATE_ID, program::ID)
    }

    pub fn resolve_for_prog(self, program_id: Pubkey) -> Result<RemoveLstKeys, FlatFeeError> {
        let state_id = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;

        self.resolve_inner(state_id, program_id)
    }

    fn resolve_inner(
        self,
        state_id: Pubkey,
        program_id: Pubkey,
    ) -> Result<RemoveLstKeys, FlatFeeError> {
        let RemoveLstFreeArgs {
            refund_rent_to,
            lst_mint,
            state_acc,
        } = self;

        if *state_acc.pubkey() != state_id {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let find_pda_args = FeeAccountFindPdaArgs {
            lst_mint,
            program_id,
        };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(RemoveLstKeys {
            manager: state.manager,
            refund_rent_to,
            fee_acc,
            lst_mint,
            state: state_id,
        })
    }
}
