use s_controller_interface::{LstState, PoolState, SControllerError};
use solana_readonly_account::ReadonlyAccountData;

use crate::try_pool_state;

pub fn sync_sol_value_with_retval(
    pool_state: &mut PoolState,
    lst_state: &mut LstState,
    returned_sol_value: u64,
) -> Result<(), SControllerError> {
    let lst_state_old_sol_value = lst_state.sol_value;
    let pool_state_new_total_sol_value = pool_state
        .total_sol_value
        .checked_sub(lst_state_old_sol_value)
        .and_then(|x| x.checked_add(returned_sol_value))
        .ok_or(SControllerError::MathError)?;

    pool_state.total_sol_value = pool_state_new_total_sol_value;
    lst_state.sol_value = returned_sol_value;

    Ok(())
}

/// TODO: confirm that theres an issue with borrowing account data across CPI boundaries
/// otherwise there isnt really a need for this fn
pub fn pool_state_total_sol_value<D: ReadonlyAccountData>(
    pool_state: D,
) -> Result<u64, SControllerError> {
    let bytes = pool_state.data();
    let deser = try_pool_state(&bytes)?;
    Ok(deser.total_sol_value)
}
