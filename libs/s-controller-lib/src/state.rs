use s_controller_interface::{LstState, PoolState, SControllerError};
use solana_readonly_account::ReadonlyAccountData;

use crate::{calc_amt_after_bps_fee, try_pool_state, CalcAmtAfterBpsFeeArgs};

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

// For nice method call syntax,
// and to reduce scope of borrowing AccountInfo.data
// to avoid CPI account data borrow failed errors
pub trait PoolStateAccount {
    /// Returns this PoolState account's current `total_sol_value`
    fn total_sol_value(&self) -> Result<u64, SControllerError>;

    /// Returns the SOL value of the protocol fees to charge
    /// on the Add/Remove liquidity operation.
    ///
    /// Args:
    /// - `lp_fees_sol_value`: SOL value of the LP fees to charge.
    ///     Calculated by taking `SOL value of LST to add or LP tokens to redeem - pricing program return value`
    fn sol_value_of_lp_protocol_fees(
        &self,
        lp_fees_sol_value: u64,
    ) -> Result<u64, SControllerError>;
}

impl<D: ReadonlyAccountData> PoolStateAccount for D {
    fn total_sol_value(&self) -> Result<u64, SControllerError> {
        let bytes = self.data();
        let deser = try_pool_state(&bytes)?;
        Ok(deser.total_sol_value)
    }

    fn sol_value_of_lp_protocol_fees(
        &self,
        lp_fees_sol_value: u64,
    ) -> Result<u64, SControllerError> {
        let bytes = self.data();
        let deser = try_pool_state(&bytes)?;
        calc_amt_after_bps_fee(CalcAmtAfterBpsFeeArgs {
            amt_before_fees: lp_fees_sol_value,
            fee_bps: deser.lp_protocol_fee_bps,
        })
    }
}
