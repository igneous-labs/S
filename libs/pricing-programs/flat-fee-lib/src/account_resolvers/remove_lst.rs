use flat_fee_interface::{FlatFeeError, ProgramState, RemoveLstKeys};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs},
    program,
    utils::try_program_state,
};

pub struct RemoveLstWithMintFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub refund_rent_to: Pubkey,
    pub lst_mint: Pubkey,
    pub state: S,
}

impl<S: KeyedAccount + ReadonlyAccountData> RemoveLstWithMintFreeArgs<S> {
    fn resolve_with_fee_acc(&self, fee_acc: Pubkey) -> Result<RemoveLstKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(RemoveLstKeys {
            manager: state.manager,
            refund_rent_to: self.refund_rent_to,
            fee_acc,
            state: program::STATE_ID,
            system_program: system_program::ID,
        })
    }

    pub fn resolve(&self) -> Result<RemoveLstKeys, FlatFeeError> {
        let find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.lst_mint,
        };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        self.resolve_with_fee_acc(fee_acc)
    }

    pub fn resolve_with_fee_acc_bump(&self, bump: u8) -> Result<RemoveLstKeys, FlatFeeError> {
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

pub struct RemoveLstFreeArgs<S: KeyedAccount + ReadonlyAccountData> {
    pub refund_rent_to: Pubkey,
    pub fee_acc: Pubkey,
    pub state: S,
}

impl<S: KeyedAccount + ReadonlyAccountData> RemoveLstFreeArgs<S> {
    pub fn resolve(&self) -> Result<RemoveLstKeys, FlatFeeError> {
        let bytes = &self.state.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(RemoveLstKeys {
            manager: state.manager,
            refund_rent_to: self.refund_rent_to,
            fee_acc: self.fee_acc,
            state: program::STATE_ID,
            system_program: system_program::ID,
        })
    }
}
