use flat_fee_interface::{FlatFeeError, ProgramState, SetLstFeeKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{
    pda::{FeeAccountCreatePdaArgs, FeeAccountFindPdaArgs},
    program::STATE_ID,
    utils::try_program_state,
};

pub struct SetLstFeeByMintFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub lst_mint: Pubkey,
    pub state_acc: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> SetLstFeeByMintFreeArgs<S> {
    fn resolve_with_fee_acc(self, fee_acc: Pubkey) -> Result<SetLstFeeKeys, FlatFeeError> {
        let SetLstFeeByMintFreeArgs {
            lst_mint: _,
            state_acc,
        } = self;

        if *state_acc.pubkey() != STATE_ID {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetLstFeeKeys {
            manager: state.manager,
            fee_acc,
            state: STATE_ID,
        })
    }

    pub fn resolve(self) -> Result<SetLstFeeKeys, FlatFeeError> {
        let find_pda_args = FeeAccountFindPdaArgs {
            lst_mint: self.lst_mint,
        };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        self.resolve_with_fee_acc(fee_acc)
    }

    pub fn resolve_with_fee_acc_bump(self, bump: u8) -> Result<SetLstFeeKeys, FlatFeeError> {
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

pub struct SetLstFeeFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub fee_acc: Pubkey,
    pub state_acc: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> SetLstFeeFreeArgs<S> {
    pub fn resolve(self) -> Result<SetLstFeeKeys, FlatFeeError> {
        let SetLstFeeFreeArgs {
            fee_acc: _,
            state_acc,
        } = self;

        if *state_acc.pubkey() != STATE_ID {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetLstFeeKeys {
            manager: state.manager,
            fee_acc: self.fee_acc,
            state: STATE_ID,
        })
    }
}
