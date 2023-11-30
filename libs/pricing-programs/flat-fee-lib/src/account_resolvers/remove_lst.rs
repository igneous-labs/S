use flat_fee_interface::{FlatFeeError, ProgramState, RemoveLstKeys};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{pda::FeeAccountFindPdaArgs, program, utils::try_program_state};

pub struct RemoveLstFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub refund_rent_to: Pubkey,
    pub state: S,
    pub lst_mint: Pubkey,
}

impl<S: KeyedAccount + ReadonlyAccountData> RemoveLstFreeArgs<S> {
    pub fn resolve(&self) -> Result<RemoveLstKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        let find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.lst_mint,
        };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        // TODO: verify

        Ok(RemoveLstKeys {
            manager: state.manager,
            refund_rent_to: self.refund_rent_to,
            fee_acc,
            state: program::STATE_ID,
            system_program: system_program::ID,
        })
    }
}
