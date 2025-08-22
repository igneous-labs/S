use sanctum_lst_list::SanctumLst;
use solana_sdk::pubkey::Pubkey;
use std::{error::Error, str::FromStr};

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
            let res = slsts
                .iter()
                .find(|lst| lst.mint == mint)
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
}
