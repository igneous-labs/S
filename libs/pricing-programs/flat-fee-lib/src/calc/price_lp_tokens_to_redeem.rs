use flat_fee_interface::FlatFeeError;

use crate::calc::BPS_DENOM;

pub fn calculate_price_lp_tokens_to_redeem(
    lp_withdrawal_fee_bps: u16,
    sol_value: u64,
) -> Result<u64, FlatFeeError> {
    let post_fee_bps: u128 = u128::try_from(BPS_DENOM)
        .map_err(|_e| FlatFeeError::MathError)?
        .checked_sub(lp_withdrawal_fee_bps.into())
        .ok_or(FlatFeeError::MathError)?;
    let result: u64 = post_fee_bps
        .checked_mul(sol_value.into())
        .and_then(|v| v.checked_div(BPS_DENOM.into()))
        .and_then(|v| v.try_into().ok())
        .ok_or(FlatFeeError::MathError)?;
    Ok(result)
}
