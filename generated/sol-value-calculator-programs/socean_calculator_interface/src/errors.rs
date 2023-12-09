use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum SoceanCalculatorError {
    #[error("Socean stake pool not yet updated for this epoch")]
    PoolNotUpdated = 0,
}
impl From<SoceanCalculatorError> for ProgramError {
    fn from(e: SoceanCalculatorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SoceanCalculatorError {
    fn type_of() -> &'static str {
        "SoceanCalculatorError"
    }
}
impl PrintProgramError for SoceanCalculatorError {
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
