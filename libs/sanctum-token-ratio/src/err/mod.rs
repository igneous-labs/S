use std::{error::Error, fmt::Display};

#[cfg(feature = "onchain")]
mod onchain;
#[cfg(feature = "onchain")]
pub use onchain::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct MathError;

impl Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathError")
    }
}

impl Error for MathError {}
