use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum FlatFeeError {
    #[error("Invalid program state data")]
    InvalidProgramStateData = 0,
    #[error("FeeAccount is not initialized for the given LST mint")]
    UnsupportedLstMint = 1,
}
impl From<FlatFeeError> for ProgramError {
    fn from(e: FlatFeeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for FlatFeeError {
    fn type_of() -> &'static str {
        "FlatFeeError"
    }
}
impl PrintProgramError for FlatFeeError {
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
