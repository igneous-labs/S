use sanctum_lst_list::SanctumLst;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
use std::{error::Error, str::FromStr};

use crate::common::{
    find_sanctum_lst_by_mint, sol_val_calc_of_sanctum_lst,
    sol_value_calculator_accounts_of_sanctum_lst,
};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum LstArg {
    SanctumLst(SanctumLst),
    Unknown(Pubkey),
}

impl LstArg {
    pub fn parse_arg(
        arg: &str,
        slsts: &[SanctumLst],
    ) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        if let Ok(mint) = Pubkey::from_str(arg) {
            let res = find_sanctum_lst_by_mint(slsts, mint)
                .map_or_else(|| Self::Unknown(mint), |s| Self::SanctumLst(s.clone()));
            return Ok(res);
        }
        let lst = slsts
            .iter()
            .find(|lst| lst.symbol == arg)
            .ok_or_else(|| format!("LST with symbol {arg} not found on list"))?;
        Ok(Self::SanctumLst(lst.clone()))
    }

    pub fn mint(&self) -> Pubkey {
        match self {
            Self::SanctumLst(lst) => lst.mint,
            Self::Unknown(pk) => *pk,
        }
    }

    pub fn token_program(&self) -> Option<Pubkey> {
        match self {
            Self::SanctumLst(lst) => Some(lst.token_program),
            Self::Unknown(_) => None,
        }
    }

    pub fn sol_val_calc_of(&self) -> Option<Pubkey> {
        match self {
            Self::SanctumLst(lst) => Some(sol_val_calc_of_sanctum_lst(lst)),
            Self::Unknown(_) => None,
        }
    }

    pub fn sol_value_calculator_accounts_of(&self) -> Option<Vec<AccountMeta>> {
        match self {
            Self::SanctumLst(lst) => Some(sol_value_calculator_accounts_of_sanctum_lst(lst)),
            Self::Unknown(_) => None,
        }
    }
}
