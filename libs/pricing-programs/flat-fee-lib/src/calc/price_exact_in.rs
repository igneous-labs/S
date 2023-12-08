use flat_fee_interface::FlatFeeError;
use sanctum_token_ratio::{U64RatioFloor, BPS_DENOMINATOR};

use super::BPS_DENOMINATOR_I16;

// wanna use an args struct here so we dont have to rmb
// whether to put input_fee_bps or output_fee_bps first?
pub fn calculate_price_exact_in(
    input_fee_bps: i16,
    output_fee_bps: i16,
    sol_value: u64,
) -> Result<u64, FlatFeeError> {
    let fee_bps = input_fee_bps
        .checked_add(output_fee_bps)
        .ok_or(FlatFeeError::MathError)?;
    let post_fee_bps: u64 = BPS_DENOMINATOR_I16
        .checked_sub(fee_bps)
        .and_then(|v| u64::try_from(v).ok())
        .ok_or(FlatFeeError::MathError)?;
    let post_fee_bps = U64RatioFloor {
        num: post_fee_bps,
        denom: BPS_DENOMINATOR,
    };
    let result = post_fee_bps
        .apply(sol_value)
        .map_err(|_e| FlatFeeError::MathError)?;
    Ok(result)
}
