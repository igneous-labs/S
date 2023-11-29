use flat_fee_interface::{FlatFeeError, ProgramState, RemoveLstKeys};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{pda::FeeAccountFindPdaArgs, utils::try_program_state};

pub struct RemoveLstFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub refund_rent_to: Pubkey,
    pub state: S,
    pub lst: Pubkey,
}

impl<S: KeyedAccount + ReadonlyAccountData> RemoveLstFreeArgs<S> {
    pub fn resolve(&self) -> Result<RemoveLstKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        let find_pda_args = FeeAccountFindPdaArgs { lst: self.lst };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        Ok(RemoveLstKeys {
            manager: state.manager,
            refund_rent_to: self.refund_rent_to,
            fee_acc,
            system_program: system_program::ID,
        })
    }
}
