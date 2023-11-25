use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum GenericPoolCalculatorError {
    #[error("stake pool program has been updated since last UpdateLastUpgradeSlot")]
    UnexpectedProgramUpgrade = 1000,
    #[error("stake pool account type is wrong")]
    WrongPoolAccountType = 1001,
    #[error("state already initialized")]
    StateAlreadyInitialized = 1002,
    #[error("calculator program is not for the given stake pool program")]
    WrongPoolProgram = 1003,
    #[error("address of CalculatorState PDA is wrong")]
    WrongCalculatorStatePda = 1004,
    #[error("Invalid calculator state data")]
    InvalidCalculatorStateData = 1005,
    #[error("Invalid stake pool program data")]
    InvalidStakePoolProgramData = 1006,
    #[error("Math error")]
    MathError = 1007,
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
