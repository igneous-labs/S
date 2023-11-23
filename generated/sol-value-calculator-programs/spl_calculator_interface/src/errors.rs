use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum SplCalculatorError {
    #[error("stake pool program has been updated since last UpdateLastUpgradeSlot")]
    UnexpectedProgramUpgrade = 0,
    #[error("stake pool account type incorrect")]
    IncorrectPoolAccountType = 1,
    #[error("stake pool not yet updated for the current epoch")]
    NotYetUpdatedForEpoch = 2,
    #[error("state already initialized")]
    AlreadyInitialized = 3,
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
