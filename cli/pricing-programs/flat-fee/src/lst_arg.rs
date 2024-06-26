use lazy_static::lazy_static;
use sanctum_lst_list::{SanctumLst, SanctumLstList};
use solana_sdk::pubkey::Pubkey;
use std::{error::Error, str::FromStr};

lazy_static! {
    pub static ref SANCTUM_LST_LIST: SanctumLstList = SanctumLstList::load();
}

#[derive(Clone, Copy, Debug)]
pub enum LstArg {
    SanctumLst(&'static SanctumLst),
    Unknown(Pubkey),
}

impl LstArg {
    pub fn parse_arg(arg: &str) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        if let Ok(mint) = Pubkey::from_str(arg) {
            let res = SANCTUM_LST_LIST
                .sanctum_lst_list
                .iter()
                .find(|lst| lst.mint == mint)
                .map_or_else(|| Self::Unknown(mint), Self::SanctumLst);
            return Ok(res);
        }
        let arg_lc = arg.to_lowercase();
        let lst = SANCTUM_LST_LIST
            .sanctum_lst_list
            .iter()
            .find(|lst| lst.symbol.to_lowercase() == arg_lc)
            .ok_or_else(|| format!("LST with symbol {arg} not found on list"))?;
        Ok(Self::SanctumLst(lst))
    }

    pub fn mint(&self) -> Pubkey {
        match self {
            Self::SanctumLst(lst) => lst.mint,
            Self::Unknown(pk) => *pk,
        }
    }
}
