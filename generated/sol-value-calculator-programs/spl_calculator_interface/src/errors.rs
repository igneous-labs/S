use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum SplCalculatorError {
    #[error("SPL stake pool not yet updated for this epoch")]
    PoolNotUpdated = 0,
}
impl From<SplCalculatorError> for ProgramError {
    fn from(e: SplCalculatorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SplCalculatorError {
    fn type_of() -> &'static str {
        "SplCalculatorError"
    }
}
impl PrintProgramError for SplCalculatorError {
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
