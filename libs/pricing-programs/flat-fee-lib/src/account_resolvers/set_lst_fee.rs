use flat_fee_interface::{FlatFeeError, ProgramState, SetLstFeeKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{pda::FeeAccountFindPdaArgs, program::STATE_ID, utils::try_program_state};

pub struct SetLstFeeByMintFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub lst_mint: Pubkey,
    pub state_acc: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> SetLstFeeByMintFreeArgs<S> {
    pub fn resolve(self) -> Result<SetLstFeeKeys, FlatFeeError> {
        let SetLstFeeByMintFreeArgs {
            lst_mint,
            state_acc,
        } = self;

        if *state_acc.pubkey() != STATE_ID {
            return Err(FlatFeeError::IncorrectProgramState);
        }

        let find_pda_args = FeeAccountFindPdaArgs { lst_mint };
        let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

        let bytes = &state_acc.data();
        let state: &ProgramState = try_program_state(bytes)?;

        Ok(SetLstFeeKeys {
            manager: state.manager,
            fee_acc,
            state: STATE_ID,
        })
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
