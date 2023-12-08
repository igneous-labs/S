use flat_fee_interface::{AddLstKeys, FlatFeeError, ProgramState};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{pda::FeeAccountFindPdaArgs, program, utils::try_program_state};

pub struct AddLstFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub payer: Pubkey,
    pub state: S,
    pub lst_mint: Pubkey,
}

// `KeyedAccount` bound not used here.
// What I mentioned about account spoofing in https://github.com/igneous-labs/S/pull/88#discussion_r1417083008
// applies here and to the other commented on instructions.
impl<S: KeyedAccount + ReadonlyAccountData> AddLstFreeArgs<S> {
    pub fn resolve(&self) -> Result<AddLstKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        let find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.lst_mint,
        };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        Ok(AddLstKeys {
            manager: state.manager,
            payer: self.payer,
            fee_acc,
            lst_mint: self.lst_mint,
            state: program::STATE_ID,
            system_program: system_program::ID,
        })
    }
}
