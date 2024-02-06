use std::{error::Error, str::FromStr};

use solana_sdk::pubkey::Pubkey;

const FLAT_FEE_IDENT: &str = "flat-fee";

#[derive(Clone, Copy, Debug)]
pub enum PricingProgArg {
    FlatFee,
    Unknown(Pubkey),
}

impl PricingProgArg {
    pub fn parse_arg(arg: &str) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        Ok(if arg == FLAT_FEE_IDENT {
            Self::FlatFee
        } else {
            Self::Unknown(Pubkey::from_str(arg)?)
        })
    }

    pub fn program_id(&self) -> Pubkey {
        match self {
            Self::FlatFee => flat_fee_lib::program::ID,
            Self::Unknown(pk) => *pk,
        }
    }
}
