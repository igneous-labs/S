use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum SControllerError {
    #[error("Invalid pool state data")]
    InvalidPoolStateData = 0,
    #[error("Invalid lst state data")]
    InvalidLstStateListData = 1,
    #[error("Invalid disable pool authority list data")]
    InvalidDisablePoolAuthorityListData = 2,
    #[error("Invalid rebalance record data")]
    InvalidRebalanceRecordData = 3,
    #[error("Math error")]
    MathError = 4,
    #[error("Pool is currently rebalancing")]
    PoolRebalancing = 5,
    #[error("Pool is currently disabled")]
    PoolDisabled = 6,
}
impl From<SControllerError> for ProgramError {
    fn from(e: SControllerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SControllerError {
    fn type_of() -> &'static str {
        "SControllerError"
    }
}
impl PrintProgramError for SControllerError {
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
