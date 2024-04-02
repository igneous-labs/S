// TODO: all generic pool calculator implementations currently assume the stake pool program is never updated,
// otherwise, get_accounts_to_update() will include the very large programdata accounts.

use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use std::collections::HashMap;

mod err;
mod lido;
mod marinade;
mod sanctum_spl;
mod sanctum_spl_multi;
mod spl;
mod traits;
mod wsol;

pub use err::*;
pub use lido::*;
pub use marinade::*;
pub use sanctum_spl::*;
pub use sanctum_spl_multi::*;
pub use spl::*;
pub use traits::*;
pub use wsol::*;

#[derive(Debug, Clone)]
pub enum KnownLstSolValCalc {
    Lido(LidoLstSolValCalc),
    Marinade(MarinadeLstSolValCalc),
    Spl(SplLstSolValCalc),
    SanctumSpl(SanctumSplLstSolValCalc),
    Wsol(WsolLstSolValCalc),
    SanctumSplMulti(SanctumSplMultiLstSolValCalc),
}

impl MutableLstSolValCalc for KnownLstSolValCalc {
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        match self {
            Self::Lido(s) => s.get_accounts_to_update(),
            Self::Marinade(s) => s.get_accounts_to_update(),
            Self::Spl(s) => s.get_accounts_to_update(),
            Self::SanctumSpl(s) => s.get_accounts_to_update(),
            Self::Wsol(s) => s.get_accounts_to_update(),
            Self::SanctumSplMulti(s) => s.get_accounts_to_update(),
        }
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> anyhow::Result<()> {
        match self {
            Self::Lido(s) => s.update(account_map),
            Self::Marinade(s) => s.update(account_map),
            Self::Spl(s) => s.update(account_map),
            Self::SanctumSpl(s) => s.update(account_map),
            Self::Wsol(s) => s.update(account_map),
            Self::SanctumSplMulti(s) => s.update(account_map),
        }
    }
}

impl LstSolValCalc for KnownLstSolValCalc {
    fn sol_value_calculator_program_id(&self) -> Pubkey {
        match self {
            Self::Lido(s) => s.sol_value_calculator_program_id(),
            Self::Marinade(s) => s.sol_value_calculator_program_id(),
            Self::Spl(s) => s.sol_value_calculator_program_id(),
            Self::SanctumSpl(s) => s.sol_value_calculator_program_id(),
            Self::Wsol(s) => s.sol_value_calculator_program_id(),
            Self::SanctumSplMulti(s) => s.sol_value_calculator_program_id(),
        }
    }

    fn lst_mint(&self) -> Pubkey {
        match self {
            Self::Lido(s) => s.lst_mint(),
            Self::Marinade(s) => s.lst_mint(),
            Self::Spl(s) => s.lst_mint(),
            Self::SanctumSpl(s) => s.lst_mint(),
            Self::Wsol(s) => s.lst_mint(),
            Self::SanctumSplMulti(s) => s.sol_value_calculator_program_id(),
        }
    }

    fn lst_to_sol(&self, lst_amount: u64) -> anyhow::Result<sanctum_token_ratio::U64ValueRange> {
        match self {
            Self::Lido(s) => s.lst_to_sol(lst_amount),
            Self::Marinade(s) => s.lst_to_sol(lst_amount),
            Self::Spl(s) => s.lst_to_sol(lst_amount),
            Self::SanctumSpl(s) => s.lst_to_sol(lst_amount),
            Self::Wsol(s) => s.lst_to_sol(lst_amount),
            Self::SanctumSplMulti(s) => s.lst_to_sol(lst_amount),
        }
    }

    fn sol_to_lst(&self, lamports: u64) -> anyhow::Result<sanctum_token_ratio::U64ValueRange> {
        match self {
            Self::Lido(s) => s.sol_to_lst(lamports),
            Self::Marinade(s) => s.sol_to_lst(lamports),
            Self::Spl(s) => s.sol_to_lst(lamports),
            Self::SanctumSpl(s) => s.sol_to_lst(lamports),
            Self::Wsol(s) => s.sol_to_lst(lamports),
            Self::SanctumSplMulti(s) => s.sol_to_lst(lamports),
        }
    }

    fn ix_accounts(&self) -> Vec<AccountMeta> {
        match self {
            Self::Lido(s) => s.ix_accounts(),
            Self::Marinade(s) => s.ix_accounts(),
            Self::Spl(s) => s.ix_accounts(),
            Self::SanctumSpl(s) => s.ix_accounts(),
            Self::Wsol(s) => s.ix_accounts(),
            Self::SanctumSplMulti(s) => s.ix_accounts(),
        }
    }
}

impl From<LidoLstSolValCalc> for KnownLstSolValCalc {
    fn from(value: LidoLstSolValCalc) -> Self {
        Self::Lido(value)
    }
}

impl From<MarinadeLstSolValCalc> for KnownLstSolValCalc {
    fn from(value: MarinadeLstSolValCalc) -> Self {
        Self::Marinade(value)
    }
}

impl From<SplLstSolValCalc> for KnownLstSolValCalc {
    fn from(value: SplLstSolValCalc) -> Self {
        Self::Spl(value)
    }
}

impl From<SanctumSplLstSolValCalc> for KnownLstSolValCalc {
    fn from(value: SanctumSplLstSolValCalc) -> Self {
        Self::SanctumSpl(value)
    }
}

impl From<WsolLstSolValCalc> for KnownLstSolValCalc {
    fn from(value: WsolLstSolValCalc) -> Self {
        Self::Wsol(value)
    }
}
