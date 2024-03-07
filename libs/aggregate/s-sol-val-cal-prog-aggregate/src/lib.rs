// TODO: all generic pool calculator implementations currently assume the stake pool program is never updated,
// otherwise, get_accounts_to_update() will include the very large programdata accounts.

use solana_program::pubkey::Pubkey;
use solana_readonly_account::ReadonlyAccountData;
use std::{collections::HashMap, error::Error};

mod err;
mod spl;
mod traits;

pub use err::*;
pub use spl::*;
pub use traits::*;

pub enum KnownLstSolValCalc {
    Spl(SplLstSolValCalc),
}

impl MutableLstSolValCalc for KnownLstSolValCalc {
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        match self {
            Self::Spl(s) => s.get_accounts_to_update(),
        }
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            Self::Spl(s) => s.update(account_map),
        }
    }
}

impl LstSolValCalc for KnownLstSolValCalc {
    fn lst_mint(&self) -> Pubkey {
        match self {
            Self::Spl(s) => s.lst_mint,
        }
    }

    fn lst_to_sol(
        &self,
        lst_amount: u64,
    ) -> Result<sanctum_token_ratio::U64ValueRange, Box<dyn Error + Send + Sync>> {
        match self {
            Self::Spl(s) => s.lst_to_sol(lst_amount),
        }
    }

    fn sol_to_lst(
        &self,
        lamports: u64,
    ) -> Result<sanctum_token_ratio::U64ValueRange, Box<dyn Error + Send + Sync>> {
        match self {
            Self::Spl(s) => s.sol_to_lst(lamports),
        }
    }

    fn ix_accounts(
        &self,
    ) -> Result<Vec<solana_program::instruction::AccountMeta>, Box<dyn Error + Send + Sync>> {
        match self {
            Self::Spl(s) => s.ix_accounts(),
        }
    }
}

impl From<SplLstSolValCalc> for KnownLstSolValCalc {
    fn from(value: SplLstSolValCalc) -> Self {
        Self::Spl(value)
    }
}
