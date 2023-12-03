use flat_fee_interface::PriceLpTokensToMintIxArgs;
use flat_fee_lib::processor::{
    process_price_lp_tokens_to_mint_unchecked, verify_price_lp_tokens_to_mint,
};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_price_lp_tokens_to_mint(
    accounts: &[AccountInfo],
    PriceLpTokensToMintIxArgs { amount, sol_value }: PriceLpTokensToMintIxArgs,
) -> ProgramResult {
    let checked = verify_price_lp_tokens_to_mint(accounts)?;
    process_price_lp_tokens_to_mint_unchecked(checked, amount, sol_value)
}
