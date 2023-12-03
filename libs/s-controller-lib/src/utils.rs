use s_controller_interface::SControllerError;

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
) -> Result<u64, SControllerError> {
    let f: u128 = final_sol_value_to_add.into();
    let l: u128 = lp_token_supply.into();
    let p: u128 = pool_total_sol_value.into();
    f.checked_mul(l)
        .and_then(|fl| fl.checked_div(p))
        .ok_or(SControllerError::MathError)
        .and_then(|x| x.try_into().map_err(|_e| SControllerError::MathError))
}
