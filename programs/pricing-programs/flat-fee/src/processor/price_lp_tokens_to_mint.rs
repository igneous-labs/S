use flat_fee_interface::PriceLpTokensToMintIxArgs;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data,
};

// flat fee program doesnt charge add liquidity fees,
// simply return passed in sol_value
pub fn process_price_lp_tokens_to_mint(
    _accounts: &[AccountInfo],
    PriceLpTokensToMintIxArgs { sol_value, .. }: PriceLpTokensToMintIxArgs,
) -> ProgramResult {
    let result_le = sol_value.to_le_bytes();
    set_return_data(&result_le);
    Ok(())
}
