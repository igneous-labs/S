use flat_fee_interface::{FlatFeeError, ProgramState, SetLstFeeKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{pda::FeeAccountFindPdaArgs, utils::try_program_state};

pub struct SetLstFeeFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub lst_mint: Pubkey,
    pub state: S,
}

impl<S: KeyedAccount + ReadonlyAccountData> SetLstFeeFreeArgs<S> {
    pub fn resolve(self) -> Result<SetLstFeeKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        let find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.lst_mint,
        };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        Ok(SetLstFeeKeys {
            fee_acc,
            manager: state.manager,
        })
    }
}
