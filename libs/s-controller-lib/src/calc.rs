use sanctum_token_ratio::{
    CeilDiv, FloorDiv, MathError, ReversibleFee, ReversibleRatio, U64BpsFee, U64Ratio,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LpTokenRateArgs {
    pub lp_token_supply: u64,
    pub pool_total_sol_value: u64,
}

/// Args:
/// - `final_sol_value`: sol_value of the LST that is being added to the pool_reserves,
///                      result of calling `PriceLpTokensToMint(LstToSol(lst_amount_to_add))`
///
/// Returns amount of LP tokens to mint to the user
pub fn calc_lp_tokens_to_mint(
    LpTokenRateArgs {
        lp_token_supply,
        pool_total_sol_value,
    }: LpTokenRateArgs,
    final_sol_value_to_add: u64,
) -> Result<u64, MathError> {
    // edge-case: if LP supply 0,
    // make it s.t. lp_token:sol_value 1:1 exchange rate
    if lp_token_supply == 0 {
        return pool_total_sol_value
            .checked_add(final_sol_value_to_add)
            .ok_or(MathError);
    }
    // edge-case: if LP supply nonzero but pool sol value 0,
    // mint amount == final_sol_value_to_add.
    // This dilutes the LPer but ensures pool can still function.
    // Should never happen.
    if pool_total_sol_value == 0 {
        return Ok(final_sol_value_to_add);
    }
    FloorDiv(U64Ratio {
        num: lp_token_supply,
        denom: pool_total_sol_value,
    })
    .apply(final_sol_value_to_add)
}

/// Returns SOL value of `lp_tokens_amount`
pub fn calc_lp_tokens_sol_value(
    LpTokenRateArgs {
        lp_token_supply,
        pool_total_sol_value,
    }: LpTokenRateArgs,
    lp_tokens_amount: u64,
) -> Result<u64, MathError> {
    if pool_total_sol_value == 0 || lp_token_supply == 0 {
        return Ok(0);
    }
    FloorDiv(U64Ratio {
        num: pool_total_sol_value,
        denom: lp_token_supply,
    })
    .apply(lp_tokens_amount)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalcAddLiquidityArgs {
    pub lst_amount: u64,

    /// Result of CPI LstToSol(lst_amount)
    pub lst_amount_sol_value: u64,

    /// Result of CPI PriceLpTokensToMint(lst_amount, sol_value_to_add).
    /// lst_amount_sol_value = lst_amount_sol_value_after_fees + protocol_fees + non_protocol_fees
    pub lst_amount_sol_value_after_fees: u64,

    pub lp_protocol_fee_bps: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalcAddLiquidityProtocolFeesResult {
    /// Amount of LST to transfer to pool_reserves
    pub to_reserves_lst_amount: u64,

    /// Amount of LST to transfer to protocol_fee_accumulator
    pub to_protocol_fees_lst_amount: u64,
}

pub fn calc_add_liquidity_protocol_fees(
    CalcAddLiquidityArgs {
        lst_amount,
        lst_amount_sol_value,
        lst_amount_sol_value_after_fees,
        lp_protocol_fee_bps,
    }: CalcAddLiquidityArgs,
) -> Result<CalcAddLiquidityProtocolFeesResult, MathError> {
    let lp_fees_sol_value = lst_amount_sol_value.saturating_sub(lst_amount_sol_value_after_fees);
    let aaf = CeilDiv(U64BpsFee::try_new(lp_protocol_fee_bps)?).apply(lp_fees_sol_value)?;
    let protocol_fees_sol_value = aaf.fee_charged();
    let to_protocol_fees_lst_amount = FloorDiv(U64Ratio {
        num: lst_amount,
        denom: lst_amount_sol_value,
    })
    .apply(protocol_fees_sol_value)?;
    let to_reserves_lst_amount = lst_amount
        .checked_sub(to_protocol_fees_lst_amount)
        .ok_or(MathError)?;
    Ok(CalcAddLiquidityProtocolFeesResult {
        to_protocol_fees_lst_amount,
        to_reserves_lst_amount,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalcRemoveLiquidityProtocolFeesArgs {
    /// Result of calc_lp_tokens_sol_value(lp_token_amount)
    pub lp_tokens_sol_value: u64,

    /// Result of CPI PriceLpTokensToRedeem(lp_token_amount, lp_tokens_sol_value)
    pub lp_tokens_sol_value_after_fees: u64,

    /// Result of CPI SolToLst(lp_tokens_sol_value_after_fees)
    pub to_user_lst_amount: u64,

    pub lp_protocol_fee_bps: u16,
}

/// Returns amount of LST to transfer to protocol_fee_accumulator
pub fn calc_remove_liquidity_protocol_fees(
    CalcRemoveLiquidityProtocolFeesArgs {
        lp_tokens_sol_value,
        lp_tokens_sol_value_after_fees,
        to_user_lst_amount,
        lp_protocol_fee_bps,
    }: CalcRemoveLiquidityProtocolFeesArgs,
) -> Result<u64, MathError> {
    let lp_fees_sol_value = lp_tokens_sol_value.saturating_sub(lp_tokens_sol_value_after_fees);
    let aaf = CeilDiv(U64BpsFee::try_new(lp_protocol_fee_bps)?).apply(lp_fees_sol_value)?;
    let protocol_fees_sol_value = aaf.fee_charged();
    let to_protocol_fees_lst_amount = FloorDiv(U64Ratio {
        num: to_user_lst_amount,
        denom: lp_tokens_sol_value_after_fees,
    })
    .apply(protocol_fees_sol_value)?;
    Ok(to_protocol_fees_lst_amount)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalcSwapProtocolFeesArgs {
    /// SOL value of LST user is transferring in to the pool
    pub in_sol_value: u64,

    /// SOL value of dst_lst_out
    pub out_sol_value: u64,

    /// Amount of dst_lst to transfer to user
    pub dst_lst_out: u64,

    pub trading_protocol_fee_bps: u16,
}

/// Returns amount of dst_lst tokens to transfer
/// from pool_reserves to protocol_fee_accumulator
pub fn calc_swap_protocol_fees(
    CalcSwapProtocolFeesArgs {
        in_sol_value,
        out_sol_value,
        dst_lst_out,
        trading_protocol_fee_bps,
    }: CalcSwapProtocolFeesArgs,
) -> Result<u64, MathError> {
    let fees_sol_value = in_sol_value.saturating_sub(out_sol_value);
    let aaf = CeilDiv(U64BpsFee::try_new(trading_protocol_fee_bps)?).apply(fees_sol_value)?;
    let protocol_fees_sol_value = aaf.fee_charged();
    let to_protocol_fees_lst_amount = FloorDiv(U64Ratio {
        num: dst_lst_out,
        denom: out_sol_value,
    })
    .apply(protocol_fees_sol_value)?;
    Ok(to_protocol_fees_lst_amount)
}
