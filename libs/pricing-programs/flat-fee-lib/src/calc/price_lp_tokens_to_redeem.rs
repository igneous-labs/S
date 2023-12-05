use flat_fee_interface::FlatFeeError;
use sanctum_token_ratio::U64BpsFeeCeil;

pub fn calculate_price_lp_tokens_to_redeem(
    lp_withdrawal_fee_bps: u16,
    sol_value: u64,
) -> Result<u64, FlatFeeError> {
    let lp_withdrawal_fee_bps = U64BpsFeeCeil(lp_withdrawal_fee_bps);
    let post_fee_amount = lp_withdrawal_fee_bps
        .apply(sol_value)
        .map_err(|_e| FlatFeeError::MathError)?;
    let result = post_fee_amount.amt_after_fee;
    Ok(result)
}
