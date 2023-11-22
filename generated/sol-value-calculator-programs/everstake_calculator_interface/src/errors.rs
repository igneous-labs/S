use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum EverstakeCalculatorError {
    #[error("stake pool program has been updated since last UpdateLastUpgradeSlot")]
    UnexpectedProgramUpgrade = 0,
    #[error("stake pool account type incorrect")]
    IncorrectPoolAccountType = 1,
    #[error("stake pool not yet updated for the current epoch")]
    NotYetUpdatedForEpoch = 2,
}
impl From<EverstakeCalculatorError> for ProgramError {
    fn from(e: EverstakeCalculatorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for EverstakeCalculatorError {
    fn type_of() -> &'static str {
        "EverstakeCalculatorError"
    }
}
impl PrintProgramError for EverstakeCalculatorError {
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
