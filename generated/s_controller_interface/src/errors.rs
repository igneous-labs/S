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
    #[error("LST with given index does not exist")]
    InvalidLstIndex = 7,
    #[error("Invalid LST reserves account")]
    InvalidReserves = 8,
    #[error("Incorrect SOL value calculator program")]
    IncorrectSolValueCalculator = 9,
    #[error("Faulty SOL value calculator program")]
    FaultySolValueCalculator = 10,
    #[error("Incorrect LST state list account")]
    IncorrectLstStateList = 11,
    #[error("Incorrect pool state account")]
    IncorrectPoolState = 12,
    #[error("Input is currently disabled for this LST")]
    LstInputDisabled = 13,
    #[error("No succeeding EndRebalance instruction found")]
    NoSucceedingEndRebalance = 14,
    #[error("Incorrect rebalance record account")]
    IncorrectRebalanceRecord = 15,
    #[error("Pool is not currently rebalancing")]
    PoolNotRebalancing = 16,
    #[error("Cannot allow loss of SOL value for pool")]
    PoolWouldLoseSolValue = 17,
    #[error(
        "Cannot remove LST when reserves or protocol fee accumulator not empty or SOL value not synced"
    )]
    LstStillHasValue = 18,
    #[error("Incorrect pricing program")]
    IncorrectPricingProgram = 19,
    #[error("Swap would exceed slippage tolerance")]
    SlippageToleranceExceeded = 20,
    #[error("Not enough liquidity complete swap")]
    NotEnoughLiquidity = 21,
    #[error("Provided list index argument is too large")]
    IndexTooLarge = 22,
    #[error("Disable Pool Authority with given index does not exist")]
    InvalidDisablePoolAuthorityIndex = 23,
    #[error("Signer is not authorized to remove given disable pool authority")]
    UnauthorizedDisablePoolAuthoritySigner = 24,
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
