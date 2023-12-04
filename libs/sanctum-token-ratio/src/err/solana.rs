#![cfg(feature = "solana")]

use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};

use super::MathError;

pub const MATH_ERROR_PROGRAM_ERROR_CODE: u32 = 696969;

impl From<MathError> for ProgramError {
    fn from(_value: MathError) -> Self {
        Self::Custom(MATH_ERROR_PROGRAM_ERROR_CODE)
    }
}

impl<T> DecodeError<T> for MathError {
    fn type_of() -> &'static str {
        "MathError"
    }
}
impl PrintProgramError for MathError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
