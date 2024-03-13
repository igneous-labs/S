use anyhow::anyhow;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use s_controller_interface::{LstState, PoolState};
use s_controller_lib::sync_sol_value_with_retval;
use s_sol_val_calc_prog_aggregate::{KnownLstSolValCalc, LstSolValCalc};
use sanctum_token_ratio::AmtsAfterFee;

use crate::LstData;

/// Returns
/// (updated pool state, update lst state, reserves balance)
pub fn apply_sync_sol_value(
    mut pool_state: PoolState,
    mut lst_state: LstState,
    LstData {
        sol_val_calc,
        reserves_balance,
        token_program: _,
    }: &LstData,
) -> anyhow::Result<(PoolState, LstState, u64)> {
    let reserves_balance = *reserves_balance
        .as_ref()
        .ok_or_else(|| anyhow!("Reserves balance not fetched"))?;
    let ret_sol_val = sol_val_calc.lst_to_sol(reserves_balance)?;
    sync_sol_value_with_retval(&mut pool_state, &mut lst_state, ret_sol_val.get_min())?;
    Ok((pool_state, lst_state, reserves_balance))
}

/// Returns (fee_amount, fee_pct)
/// fee_pct is [0.0, 1.0], not [0, 100],
/// so 0.1 (NOT 10.0) means 10%
pub fn calc_quote_fees(
    sol_value_amts: AmtsAfterFee,
    sol_val_calc: &KnownLstSolValCalc,
) -> anyhow::Result<(u64, Decimal)> {
    let fee_amount_sol = sol_value_amts.fee_charged();
    let fee_pct_num = Decimal::from_u64(fee_amount_sol)
        .ok_or_else(|| anyhow!("Decimal conv error fees_charged"))?;
    let fee_pct_denom = Decimal::from_u64(sol_value_amts.amt_before_fee()?)
        .ok_or_else(|| anyhow!("Decimal conv error amt_before_fee"))?;
    let fee_pct = fee_pct_num
        .checked_div(fee_pct_denom)
        .ok_or_else(|| anyhow!("Decimal fee_pct div err"))?;
    let fee_amount = sol_val_calc.sol_to_lst(fee_amount_sol)?.get_min();
    Ok((fee_amount, fee_pct))
}
