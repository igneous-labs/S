use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum GenericPoolCalculatorError {
    #[error("stake pool program has been updated since last UpdateLastUpgradeSlot")]
    UnexpectedProgramUpgrade = 0,
    #[error("stake pool account type incorrect")]
    IncorrectPoolAccountType = 1,
    #[error("state already initialized")]
    StateAlreadyInitialized = 2,
}
impl From<GenericPoolCalculatorError> for ProgramError {
    fn from(e: GenericPoolCalculatorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for GenericPoolCalculatorError {
    fn type_of() -> &'static str {
        "GenericPoolCalculatorError"
    }
}
impl PrintProgramError for GenericPoolCalculatorError {
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
