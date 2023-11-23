use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum WsolCalculatorError {
    #[error("Mint passed in is not the wSOL mint")]
    IncorrectMint = 0,
}
impl From<WsolCalculatorError> for ProgramError {
    fn from(e: WsolCalculatorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for WsolCalculatorError {
    fn type_of() -> &'static str {
        "WsolCalculatorError"
    }
}
impl PrintProgramError for WsolCalculatorError {
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
