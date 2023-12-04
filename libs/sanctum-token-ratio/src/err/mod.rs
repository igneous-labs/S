use std::{error::Error, fmt::Display};

mod solana;

pub use solana::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct MathError;

impl Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathError")
    }
}

impl Error for MathError {}
