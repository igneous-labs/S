use s_controller_interface::{LstState, PoolState, SControllerError};
use solana_readonly_account::ReadonlyAccountData;

use crate::{try_pool_state, U8Bool};

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

/// For nice method call syntax both onchain and offchain,
/// and to reduce scope of borrowing AccountInfo.data
/// to avoid CPI account data borrow failed errors.
/// Hopefully bytemuck means pointer casting of data to &PoolState is cheap.
pub trait PoolStateAccount {
    /// Returns this PoolState account's current `total_sol_value`
    fn total_sol_value(&self) -> Result<u64, SControllerError>;

    fn lp_protocol_fee_bps(&self) -> Result<u16, SControllerError>;

    fn is_disabled(&self) -> Result<bool, SControllerError>;
}

impl<D: ReadonlyAccountData> PoolStateAccount for D {
    fn total_sol_value(&self) -> Result<u64, SControllerError> {
        let bytes = self.data();
        let deser = try_pool_state(&bytes)?;
        Ok(deser.total_sol_value)
    }

    fn lp_protocol_fee_bps(&self) -> Result<u16, SControllerError> {
        let bytes = self.data();
        let deser = try_pool_state(&bytes)?;
        Ok(deser.lp_protocol_fee_bps)
    }

    fn is_disabled(&self) -> Result<bool, SControllerError> {
        let bytes = self.data();
        let deser = try_pool_state(&bytes)?;
        Ok(U8Bool(deser.is_disabled).is_true())
    }
}
