use flat_fee_interface::{FlatFeeError, ProgramState, SetLstFeeKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs},
    program,
    utils::try_program_state,
};

pub struct SetLstFeeWithMintFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub lst_mint: Pubkey,
    pub state: S,
}

// `KeyedAccount` bound not used here
impl<S: KeyedAccount + ReadonlyAccountData> SetLstFeeWithMintFreeArgs<S> {
    fn resolve_with_fee_acc(&self, fee_acc: Pubkey) -> Result<SetLstFeeKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetLstFeeKeys {
            manager: state.manager,
            fee_acc,
            state: program::STATE_ID,
        })
    }

    pub fn resolve(&self) -> Result<SetLstFeeKeys, FlatFeeError> {
        let find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.lst_mint,
        };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        self.resolve_with_fee_acc(fee_acc)
    }

    pub fn resolve_with_fee_acc_bump(&self, bump: u8) -> Result<SetLstFeeKeys, FlatFeeError> {
        let create_pda_args = FeeAccountCreatePdaArgs {
            find_pda_args: FeeAccountFindPdaArgs {
                lst_mint: self.lst_mint,
            },
            bump: [bump],
        };
        let fee_acc = create_pda_args
            .get_fee_account_address()
            .map_err(|_e| FlatFeeError::UnsupportedLstMint)?;

        self.resolve_with_fee_acc(fee_acc)
    }
}

pub struct SetLstFeeFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub fee_acc: Pubkey,
    pub state: S,
}

// `KeyedAccount` bound not used here
impl<S: KeyedAccount + ReadonlyAccountData> SetLstFeeFreeArgs<S> {
    pub fn resolve(&self) -> Result<SetLstFeeKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetLstFeeKeys {
            manager: state.manager,
            fee_acc: self.fee_acc,
            state: program::STATE_ID,
        })
    }
}
