use flat_fee_interface::FlatFeeError;
use sanctum_token_ratio::{U64RatioFloor, BPS_DENOMINATOR};

use super::BPS_DENOMINATOR_I16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalculatePriceExactInArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
    pub sol_value: u64,
}

/// Returns `out_sol_value`
pub fn calculate_price_exact_in(
    CalculatePriceExactInArgs {
        input_fee_bps,
        output_fee_bps,
        sol_value,
    }: CalculatePriceExactInArgs,
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
