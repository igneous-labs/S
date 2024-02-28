use flat_fee_interface::FlatFeeError;
use sanctum_token_ratio::ReversibleRatio;

use super::common::{out_sol_value_ratio, OutSolValueRatioArgs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalculatePriceExactOutArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
    pub out_sol_value: u64,
}

/// Returns `in_sol_value`
pub fn calculate_price_exact_out(
    CalculatePriceExactOutArgs {
        input_fee_bps,
        output_fee_bps,
        out_sol_value,
    }: CalculatePriceExactOutArgs,
) -> Result<u64, FlatFeeError> {
    Ok(out_sol_value_ratio(OutSolValueRatioArgs {
        input_fee_bps,
        output_fee_bps,
    })?
    .reverse(out_sol_value)
    .map_err(|_e| FlatFeeError::MathError)?
    .get_max())
}
