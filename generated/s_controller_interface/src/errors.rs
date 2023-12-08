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
    #[error("Pool is currently enabled ")]
    PoolEnabled = 7,
    #[error("LST with given index does not exist")]
    InvalidLstIndex = 8,
    #[error("Invalid LST reserves account")]
    InvalidReserves = 9,
    #[error("Incorrect SOL value calculator program")]
    IncorrectSolValueCalculator = 10,
    #[error("Faulty SOL value calculator program")]
    FaultySolValueCalculator = 11,
    #[error("Incorrect LST state list account")]
    IncorrectLstStateList = 12,
    #[error("Incorrect pool state account")]
    IncorrectPoolState = 13,
    #[error("Input is currently disabled for this LST")]
    LstInputDisabled = 14,
    #[error("No succeeding EndRebalance instruction found")]
    NoSucceedingEndRebalance = 15,
    #[error("Incorrect rebalance record account")]
    IncorrectRebalanceRecord = 16,
    #[error("Pool is not currently rebalancing")]
    PoolNotRebalancing = 17,
    #[error("Cannot allow loss of SOL value for pool")]
    PoolWouldLoseSolValue = 18,
    #[error(
        "Cannot remove LST when reserves or protocol fee accumulator not empty or SOL value not synced"
    )]
    LstStillHasValue = 19,
    #[error("Incorrect pricing program")]
    IncorrectPricingProgram = 20,
    #[error("Swap would exceed slippage tolerance")]
    SlippageToleranceExceeded = 21,
    #[error("Not enough liquidity complete swap")]
    NotEnoughLiquidity = 22,
    #[error("Provided list index argument is too large")]
    IndexTooLarge = 23,
    #[error("Disable Pool Authority with given index does not exist")]
    InvalidDisablePoolAuthorityIndex = 24,
    #[error("Signer is not authorized to operate on given disable pool authority")]
    UnauthorizedDisablePoolAuthoritySigner = 25,
    #[error("Given disable pool authority is not valid")]
    InvalidDisablePoolAuthority = 26,
    #[error("Signer is not authorized to set rebalance authority")]
    UnauthorizedSetRebalanceAuthoritySigner = 27,
    #[error("Incorrect disable pool authority list account")]
    IncorrectDisablePoolAuthorityList = 28,
    #[error("Attempting to set a fee over 100%")]
    FeeTooHigh = 29,
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
