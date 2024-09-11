use std::{error::Error, str::FromStr};

use solana_sdk::pubkey::Pubkey;
use spl_calculator_lib::{
    sanctum_spl_multi_sol_val_calc_program, sanctum_spl_sol_val_calc_program,
};

#[derive(Clone, Copy, Debug)]
pub enum SolValCalcArg {
    Lido,
    Marinade,
    SanctumSpl,
    SanctumSplMulti,
    Spl,
    Wsol,
    Unknown(Pubkey),
}

impl SolValCalcArg {
    pub const HELP_STR: &'static str = "A SOL Value Calculator Program. Can either be a program ID pubkey or one of the following known programs:
- lido
- marinade
- sanctum-spl
- sanctum-spl-multi
- spl
- wsol";

    pub fn parse_arg(arg: &str) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        Ok(match arg {
            "lido" => Self::Lido,
            "marinade" => Self::Marinade,
            "sanctum-spl" => Self::SanctumSpl,
            "sanctum-spl-multi" => Self::SanctumSplMulti,
            "spl" => Self::Spl,
            "wsol" => Self::Wsol,
            _ => {
                let pk = Pubkey::from_str(arg)?;
                match pk {
                    lido_calculator_lib::program::ID => Self::Lido,
                    marinade_calculator_lib::program::ID => Self::Marinade,
                    sanctum_spl_sol_val_calc_program::ID => Self::SanctumSpl,
                    sanctum_spl_multi_sol_val_calc_program::ID => Self::SanctumSplMulti,
                    spl_calculator_lib::program::ID => Self::Spl,
                    wsol_calculator_lib::program::ID => Self::Wsol,
                    _ => Self::Unknown(pk),
                }
            }
        })
    }

    pub fn program_id(&self) -> Pubkey {
        match self {
            Self::Lido => lido_calculator_lib::program::ID,
            Self::Marinade => marinade_calculator_lib::program::ID,
            Self::SanctumSpl => sanctum_spl_sol_val_calc_program::ID,
            Self::SanctumSplMulti => sanctum_spl_multi_sol_val_calc_program::ID,
            Self::Spl => spl_calculator_lib::program::ID,
            Self::Wsol => wsol_calculator_lib::program::ID,
            Self::Unknown(pk) => *pk,
        }
    }
}
