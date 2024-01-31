use flat_fee_interface::FlatFeeError;
use sanctum_token_ratio::{FloorDiv, U64Ratio, BPS_DENOMINATOR};

use super::BPS_DENOMINATOR_I16;

#[derive(Clone, Copy)]
pub struct OutSolValueRatioArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
}

/// Returns the ratio that returns out_sol_value
/// when applied to in_sol_value
pub fn out_sol_value_ratio(
    OutSolValueRatioArgs {
        input_fee_bps,
        output_fee_bps,
    }: OutSolValueRatioArgs,
) -> Result<FloorDiv<U64Ratio<u16, u16>>, FlatFeeError> {
    let fee_bps = input_fee_bps
        .checked_add(output_fee_bps)
        .ok_or(FlatFeeError::MathError)?;
    // post_fee_bps = 10_000 - fee_bps
    // out_sol_value = floor(in_sol_value * post_fee_bps / 10_000)
    // i16 signed subtraction:
    // - rebates are allowed (post_fee_bps > 10_000)
    // - however, >100% fees will error (post_fee_bps < 0)
    let post_fee_bps: u16 = BPS_DENOMINATOR_I16
        .checked_sub(fee_bps)
        .and_then(|v| v.try_into().ok())
        .ok_or(FlatFeeError::MathError)?;
    Ok(FloorDiv(U64Ratio {
        num: post_fee_bps,
        denom: BPS_DENOMINATOR,
    }))
}
