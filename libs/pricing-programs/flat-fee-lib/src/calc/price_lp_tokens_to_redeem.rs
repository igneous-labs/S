use flat_fee_interface::FlatFeeError;
use sanctum_token_ratio::{CeilDiv, ReversibleFee, U64BpsFee};

pub fn calculate_price_lp_tokens_to_redeem(
    lp_withdrawal_fee_bps: u16,
    sol_value: u64,
) -> Result<u64, FlatFeeError> {
    U64BpsFee::try_new(lp_withdrawal_fee_bps)
        .map(CeilDiv)
        .and_then(|f| f.apply(sol_value))
        .map(|aaf| aaf.amt_after_fee())
        .map_err(|_e| FlatFeeError::MathError)
}
