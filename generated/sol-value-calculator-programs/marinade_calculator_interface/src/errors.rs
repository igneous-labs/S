use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum MarinadeCalculatorError {
    #[error("Marinade program is currently paused")]
    MarinadePaused = 0,
}
impl From<MarinadeCalculatorError> for ProgramError {
    fn from(e: MarinadeCalculatorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for MarinadeCalculatorError {
    fn type_of() -> &'static str {
        "MarinadeCalculatorError"
    }
}
impl PrintProgramError for MarinadeCalculatorError {
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
