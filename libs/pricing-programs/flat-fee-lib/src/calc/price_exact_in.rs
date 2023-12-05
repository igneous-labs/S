use flat_fee_interface::FlatFeeError;

use super::{BPS_DENOM_I16, BPS_DENOM_U128};

pub fn calculate_price_exact_in(
    input_fee_bps: i16,
    output_fee_bps: i16,
    sol_value: u64,
) -> Result<u64, FlatFeeError> {
    let fee_bps = input_fee_bps
        .checked_add(output_fee_bps)
        .ok_or(FlatFeeError::MathError)?;
    let post_fee_bps: u128 = BPS_DENOM_I16
        .checked_sub(fee_bps)
        .ok_or(FlatFeeError::MathError)?
        .try_into()
        .map_err(|_e| FlatFeeError::MathError)?;
    let result: u64 = post_fee_bps
        .checked_mul(sol_value.into())
        .and_then(|v| v.checked_div(BPS_DENOM_U128))
        .and_then(|v| v.try_into().ok())
        .ok_or(FlatFeeError::MathError)?;
    Ok(result)
}
