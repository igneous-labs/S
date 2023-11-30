//! Common types that unifies LstToSol and SolToLst Accounts and Keys

use generic_pool_calculator_interface::{
    LstToSolAccounts, LstToSolKeys, SolToLstAccounts, SolToLstKeys,
};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

pub struct LstSolCommonAccounts<'me, 'info> {
    ///The LST mint
    pub lst_mint: &'me AccountInfo<'info>,
    ///The CalculatorState PDA
    pub state: &'me AccountInfo<'info>,
    ///The main stake pool state account
    pub pool_state: &'me AccountInfo<'info>,
    ///The stake pool program
    pub pool_program: &'me AccountInfo<'info>,
    ///The stake pool program executable data
    pub pool_program_data: &'me AccountInfo<'info>,
}

impl<'me, 'info> From<LstSolCommonAccounts<'me, 'info>> for LstToSolAccounts<'me, 'info> {
    fn from(
        LstSolCommonAccounts {
            lst_mint,
            state,
            pool_state,
            pool_program,
            pool_program_data,
        }: LstSolCommonAccounts<'me, 'info>,
    ) -> Self {
        Self {
            lst_mint,
            state,
            pool_state,
            pool_program,
            pool_program_data,
        }
    }
}

impl<'me, 'info> From<LstSolCommonAccounts<'me, 'info>> for SolToLstAccounts<'me, 'info> {
    fn from(
        LstSolCommonAccounts {
            lst_mint,
            state,
            pool_state,
            pool_program,
            pool_program_data,
        }: LstSolCommonAccounts<'me, 'info>,
    ) -> Self {
        Self {
            lst_mint,
            state,
            pool_state,
            pool_program,
            pool_program_data,
        }
    }
}

pub struct LstSolCommonKeys {
    ///The LST mint
    pub lst_mint: Pubkey,
    ///The CalculatorState PDA
    pub state: Pubkey,
    ///The main stake pool state account
    pub pool_state: Pubkey,
    ///The stake pool program
    pub pool_program: Pubkey,
    ///The stake pool program executable data
    pub pool_program_data: Pubkey,
}

impl From<LstSolCommonKeys> for LstToSolKeys {
    fn from(
        LstSolCommonKeys {
            lst_mint,
            state,
            pool_state,
            pool_program,
            pool_program_data,
        }: LstSolCommonKeys,
    ) -> Self {
        Self {
            lst_mint,
            state,
            pool_state,
            pool_program,
            pool_program_data,
        }
    }
}

impl From<LstSolCommonKeys> for SolToLstKeys {
    fn from(
        LstSolCommonKeys {
            lst_mint,
            state,
            pool_state,
            pool_program,
            pool_program_data,
        }: LstSolCommonKeys,
    ) -> Self {
        Self {
            lst_mint,
            state,
            pool_state,
            pool_program,
            pool_program_data,
        }
    }
}
