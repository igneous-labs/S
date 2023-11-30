use s_controller_interface::{PoolState, SControllerError};
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
