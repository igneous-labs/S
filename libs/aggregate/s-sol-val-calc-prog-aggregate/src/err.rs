use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub enum LstSolValCalcErr {
    WrongLstSolValCalc,
}

impl Display for LstSolValCalcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongLstSolValCalc => f.write_str("wrong LstSolValCalc variant"),
        }
    }
}

impl Error for LstSolValCalcErr {}
