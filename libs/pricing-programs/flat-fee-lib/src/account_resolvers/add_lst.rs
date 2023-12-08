use flat_fee_interface::{AddLstKeys, FlatFeeError, ProgramState};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs},
    program::STATE_ID,
    utils::try_program_state,
};

pub struct AddLstFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub payer: Pubkey,
    pub state_acc: S,
    pub lst_mint: Pubkey,
}

impl<S: KeyedAccount + ReadonlyAccountData> AddLstFreeArgs<S> {
    pub fn resolve(self) -> Result<(AddLstKeys, FeeAccountCreatePdaArgs), FlatFeeError> {
        let AddLstFreeArgs {
            payer,
            state_acc,
            lst_mint,
        } = self;

        if *state_acc.key() != STATE_ID {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        let find_pda_args = FeeAccountFindPdaArgs { lst_mint };
        let (fee_acc, bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        Ok((
            AddLstKeys {
                manager: state.manager,
                payer,
                fee_acc,
                lst_mint,
                state: STATE_ID,
                system_program: system_program::ID,
            },
            FeeAccountCreatePdaArgs {
                find_pda_args,
                bump: [bump],
            },
        ))
    }
}
