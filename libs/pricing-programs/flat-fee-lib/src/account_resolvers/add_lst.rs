use flat_fee_interface::{AddLstKeys, FlatFeeError, ProgramState};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{pda::FeeAccountFindPdaArgs, utils::try_program_state};

pub struct AddLstFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub payer: Pubkey,
    pub state: S,
    pub lst: Pubkey,
}

impl<S: KeyedAccount + ReadonlyAccountData> AddLstFreeArgs<S> {
    pub fn resolve(&self) -> Result<AddLstKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        let find_pda_args = FeeAccountFindPdaArgs { lst: self.lst };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        Ok(AddLstKeys {
            manager: state.manager,
            payer: self.payer,
            fee_acc,
            system_program: system_program::ID,
        })
    }
}
