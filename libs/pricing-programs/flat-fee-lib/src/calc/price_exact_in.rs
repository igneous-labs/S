use flat_fee_interface::FlatFeeError;
use sanctum_token_ratio::ReversibleRatio;

use super::common::{out_sol_value_ratio, OutSolValueRatioArgs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalculatePriceExactInArgs {
    pub input_fee_bps: i16,
    pub output_fee_bps: i16,
    pub in_sol_value: u64,
}

/// Returns `out_sol_value`
pub fn calculate_price_exact_in(
    CalculatePriceExactInArgs {
        input_fee_bps,
        output_fee_bps,
        in_sol_value,
    }: CalculatePriceExactInArgs,
) -> Result<u64, FlatFeeError> {
    out_sol_value_ratio(OutSolValueRatioArgs {
        input_fee_bps,
        output_fee_bps,
    })?
    .apply(in_sol_value)
    .map_err(|_e| FlatFeeError::MathError)
}
