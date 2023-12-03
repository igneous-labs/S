use flat_fee_interface::PriceLpTokensToRedeemIxArgs;
use flat_fee_lib::processor::{
    process_price_lp_tokens_to_redeem_unchecked, verify_price_lp_tokens_to_redeem,
};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_price_lp_tokens_to_redeem(
    accounts: &[AccountInfo],
    PriceLpTokensToRedeemIxArgs { amount, sol_value }: PriceLpTokensToRedeemIxArgs,
) -> ProgramResult {
    let checked = verify_price_lp_tokens_to_redeem(accounts)?;
    process_price_lp_tokens_to_redeem_unchecked(checked, amount, sol_value)
}
