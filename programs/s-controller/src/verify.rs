//! Common verification functions used across multiple instruction processors

use s_controller_interface::{LstState, PoolState, SControllerError};
use s_controller_lib::U8Bool;

pub fn verify_not_rebalancing_and_not_disabled(
    pool_state: &PoolState,
) -> Result<(), SControllerError> {
    if U8Bool(pool_state.is_rebalancing).is_true() {
        return Err(SControllerError::PoolRebalancing);
    }
    if U8Bool(pool_state.is_disabled).is_true() {
        return Err(SControllerError::PoolDisabled);
    }
    Ok(())
}

pub fn verify_lst_input_not_disabled(lst_state: &LstState) -> Result<(), SControllerError> {
    if U8Bool(lst_state.is_input_disabled).is_true() {
        return Err(SControllerError::LstInputDisabled);
    }
    Ok(())
}
