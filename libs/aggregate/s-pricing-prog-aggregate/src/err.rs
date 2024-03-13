use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PricingProgErr {
    UnknownPricingProg,
    WrongPricingProg,
}

impl Display for PricingProgErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPricingProg => f.write_str("Unknown pricing program"),
            Self::WrongPricingProg => f.write_str("Wrong pricing program"),
        }
    }
}

impl Error for PricingProgErr {}
