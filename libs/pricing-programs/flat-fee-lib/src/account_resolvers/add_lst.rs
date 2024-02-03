use flat_fee_interface::{AddLstKeys, FlatFeeError, ProgramState};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs, ProgramStateFindPdaArgs},
    program::{self, STATE_ID},
    utils::try_program_state,
};

pub struct AddLstFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub payer: Pubkey,
    pub state_acc: S,
    pub lst_mint: Pubkey,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> AddLstFreeArgs<S> {
    pub fn resolve(self) -> Result<(AddLstKeys, FeeAccountCreatePdaArgs), FlatFeeError> {
        self.resolve_inner(STATE_ID, program::ID)
    }

    pub fn resolve_for_prog(
        self,
        program_id: Pubkey,
    ) -> Result<(AddLstKeys, FeeAccountCreatePdaArgs), FlatFeeError> {
        let state_id = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;

        self.resolve_inner(state_id, program_id)
    }

    pub fn resolve_inner(
        self,
        state_id: Pubkey,
        program_id: Pubkey,
    ) -> Result<(AddLstKeys, FeeAccountCreatePdaArgs), FlatFeeError> {
        let Self {
            payer,
            state_acc,
            lst_mint,
        } = self;

        if *state_acc.pubkey() != state_id {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        let find_pda_args = FeeAccountFindPdaArgs {
            lst_mint,
            program_id,
        };
        let (fee_acc, bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        Ok((
            AddLstKeys {
                manager: state.manager,
                payer,
                fee_acc,
                lst_mint,
                state: state_id,
                system_program: system_program::ID,
            },
            FeeAccountCreatePdaArgs {
                find_pda_args,
                bump,
            },
        ))
    }
}
