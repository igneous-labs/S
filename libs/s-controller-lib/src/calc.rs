use sanctum_token_ratio::{AmtsAfterFee, MathError, U64BpsFeeCeil, U64RatioFloor};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LpTokenRateArgs {
    pub lp_token_supply: u64,
    pub pool_total_sol_value: u64,
}

/// Args:
/// - `final_sol_value`: result of calling `PriceLpTokensToMint(LstToSol(lst_amount_to_add))`
///
/// Returns amount of LP tokens to mint
pub fn calc_lp_tokens_to_mint(
    LpTokenRateArgs {
        lp_token_supply,
        pool_total_sol_value,
    }: LpTokenRateArgs,
    final_sol_value_to_add: u64,
) -> Result<u64, MathError> {
    // edge-case: 0, just do 1:1
    if pool_total_sol_value == 0 || lp_token_supply == 0 {
        return Ok(final_sol_value_to_add);
    }
    let res = U64RatioFloor {
        num: lp_token_supply,
        denom: pool_total_sol_value,
    }
    .apply(final_sol_value_to_add)?;
    Ok(res)
}

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
    let res = U64RatioFloor {
        num: pool_total_sol_value,
        denom: lp_token_supply,
    }
    .apply(lp_tokens_amount)?;
    Ok(res)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalcAddLiquidityArgs {
    pub lst_amount: u64,

    /// Result of CPI LstToSol(lst_amount)
    pub lst_amount_sol_value: u64,

    /// Result of CPI PriceLpTokensToMint(lst_amount, sol_value_to_add)
    pub lst_amount_sol_value_after_fees: u64,

    pub lp_protocol_fee_bps: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalcAddLiquidityResult {
    /// Amount of LST to transfer to pool_reserves
    pub to_reserves_lst_amount: u64,

    /// Amount of LST to transfer to protocol_fee_accumulator
    pub to_protocol_fees_lst_amount: u64,
}

pub fn calc_add_liquidity(
    CalcAddLiquidityArgs {
        lst_amount,
        lst_amount_sol_value,
        lst_amount_sol_value_after_fees,
        lp_protocol_fee_bps,
    }: CalcAddLiquidityArgs,
) -> Result<CalcAddLiquidityResult, MathError> {
    let lp_fees_sol_value = lst_amount_sol_value.saturating_sub(lst_amount_sol_value_after_fees);
    let AmtsAfterFee {
        fees_charged: protocol_fees_sol_value,
        ..
    } = U64BpsFeeCeil(lp_protocol_fee_bps).apply(lp_fees_sol_value)?;
    let to_protocol_fees_lst_amount = U64RatioFloor {
        num: lst_amount,
        denom: lst_amount_sol_value,
    }
    .apply(protocol_fees_sol_value)?;
    let to_reserves_lst_amount = lst_amount
        .checked_sub(to_protocol_fees_lst_amount)
        .ok_or(MathError)?;
    Ok(CalcAddLiquidityResult {
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
    let AmtsAfterFee {
        fees_charged: protocol_fees_sol_value,
        ..
    } = U64BpsFeeCeil(lp_protocol_fee_bps).apply(lp_fees_sol_value)?;
    let to_protocol_fees_lst_amount = U64RatioFloor {
        num: to_user_lst_amount,
        denom: lp_tokens_sol_value_after_fees,
    }
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
    let AmtsAfterFee {
        fees_charged: protocol_fees_sol_value,
        ..
    } = U64BpsFeeCeil(trading_protocol_fee_bps).apply(fees_sol_value)?;
    let to_protocol_fees_lst_amount = U64RatioFloor {
        num: dst_lst_out,
        denom: out_sol_value,
    }
    .apply(protocol_fees_sol_value)?;
    Ok(to_protocol_fees_lst_amount)
}
