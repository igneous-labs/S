use sanctum_token_ratio::U64RatioFloor;
use solana_program::program_error::ProgramError;

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
) -> Result<u64, ProgramError> {
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
