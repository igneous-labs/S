use flat_fee_interface::FlatFeeError;

use crate::BPS_DENOM;

pub fn calculate_price_exact_out(
    input_fee_bps: i16,
    output_fee_bps: i16,
    sol_value: u64,
) -> Result<u64, FlatFeeError> {
    let fee_bps = input_fee_bps
        .checked_add(output_fee_bps)
        .ok_or(FlatFeeError::MathError)?;
    let post_fee_bps: u128 = i16::try_from(BPS_DENOM)
        .map_err(|_e| FlatFeeError::MathError)?
        .checked_sub(fee_bps)
        .ok_or(FlatFeeError::MathError)?
        .try_into()
        .map_err(|_e| FlatFeeError::MathError)?;
    let result: u64 = u128::try_from(BPS_DENOM)
        .map_err(|_e| FlatFeeError::MathError)?
        .checked_mul(sol_value.into())
        .and_then(|v| v.checked_div(post_fee_bps))
        .and_then(|v| v.try_into().ok())
        .ok_or(FlatFeeError::MathError)?;
    Ok(result)
}
