use flat_fee_interface::{
    price_lp_tokens_to_redeem_verify_account_keys,
    price_lp_tokens_to_redeem_verify_account_privileges, PriceLpTokensToRedeemAccounts,
    PriceLpTokensToRedeemKeys,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::account_resolvers::PriceLpTokensToRedeemFreeArgs;

pub fn process_price_lp_tokens_to_redeem_unchecked(
    PriceLpTokensToRedeemAccounts {
        output_lst_mint: _,
        state: _,
    }: PriceLpTokensToRedeemAccounts,
    _amount: u64,
    _sol_value: u64,
) -> ProgramResult {
    todo!()

    // TODO: calculate sol_value amount of output lst to redeem lp tokens for
    // TODO: set return value
}

pub fn verify_price_lp_tokens_to_redeem<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<PriceLpTokensToRedeemAccounts<'me, 'info>, ProgramError> {
    let actual: PriceLpTokensToRedeemAccounts = load_accounts(accounts)?;

    let free_args = PriceLpTokensToRedeemFreeArgs {
        output_lst_mint: *actual.output_lst_mint.key,
    };
    let expected: PriceLpTokensToRedeemKeys = free_args.resolve();

    price_lp_tokens_to_redeem_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    price_lp_tokens_to_redeem_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
