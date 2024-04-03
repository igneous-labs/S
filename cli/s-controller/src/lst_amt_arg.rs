use std::{error::Error, fmt::Display};

use solana_sdk::native_token::{lamports_to_sol, sol_to_lamports};

#[derive(Clone, Copy, Debug)]
pub enum LstAmtArg {
    All,
    Amt(u64),
}

impl LstAmtArg {
    pub fn parse_arg(arg: &str) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        Ok(match arg.to_lowercase().as_str() {
            "all" => Self::All,
            _ => {
                let sol: f64 = arg.parse()?;
                Self::Amt(sol_to_lamports(sol))
            }
        })
    }
}

impl Display for LstAmtArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => f.write_str("all"),
            Self::Amt(v) => write!(f, "{}", lamports_to_sol(*v)),
        }
    }
}
