use s_controller_interface::SControllerError;

use crate::BPS_DENOMINATOR;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LpTokenRateArgs {
    pub lp_token_supply: u64,
    pub pool_total_sol_value: u64,
}

/// Args:
/// - `final_sol_value`: result of calling `PriceLpTokensToMint(LstToSol(lst_amount_to_add))`
///
/// Returns amount of LP tokens to mint
pub fn calc_lp_tokens_to_mint(
    LpTokenRateArgs {
        lp_token_supply,
        pool_total_sol_value,
    }: LpTokenRateArgs,
    final_sol_value_to_add: u64,
) -> Result<u64, SControllerError> {
    if pool_total_sol_value == 0 || lp_token_supply == 0 {
        return Ok(final_sol_value_to_add);
    }
    let f: u128 = final_sol_value_to_add.into();
    let l: u128 = lp_token_supply.into();
    let p: u128 = pool_total_sol_value.into();
    f.checked_mul(l)
        .and_then(|fl| fl.checked_div(p))
        .ok_or(SControllerError::MathError)
        .and_then(|x| x.try_into().map_err(|_e| SControllerError::MathError))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalcAmtAfterBpsFeeArgs {
    pub amt_before_fees: u64,
    pub fee_bps: u16,
}

/// Returns final amount after `fee_bps` fee is charged
pub fn calc_amt_after_bps_fee(
    CalcAmtAfterBpsFeeArgs {
        amt_before_fees,
        fee_bps,
    }: CalcAmtAfterBpsFeeArgs,
) -> Result<u64, SControllerError> {
    let x: u128 = amt_before_fees.into();
    let n: u128 = BPS_DENOMINATOR
        .checked_sub(fee_bps)
        .ok_or(SControllerError::MathError)?
        .into();
    let d: u128 = BPS_DENOMINATOR.into();
    x.checked_mul(n)
        .map(|xn| xn / d)
        .ok_or(SControllerError::MathError)
        .and_then(|res| res.try_into().map_err(|_e| SControllerError::MathError))
}
