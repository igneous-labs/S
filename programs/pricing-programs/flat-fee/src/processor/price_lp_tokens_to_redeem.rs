use flat_fee_interface::{
    price_lp_tokens_to_redeem_verify_account_keys,
    price_lp_tokens_to_redeem_verify_account_privileges, PriceLpTokensToRedeemAccounts,
    PriceLpTokensToRedeemIxArgs, PriceLpTokensToRedeemKeys,
};
use flat_fee_lib::{
    account_resolvers::PriceLpTokensToRedeemFreeArgs, calc::calculate_price_lp_tokens_to_redeem,
    utils::try_program_state,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data,
    program_error::ProgramError,
};

pub fn process_price_lp_tokens_to_redeem(
    accounts: &[AccountInfo],
    PriceLpTokensToRedeemIxArgs {
        amount: _,
        sol_value,
    }: PriceLpTokensToRedeemIxArgs,
) -> ProgramResult {
    let PriceLpTokensToRedeemAccounts {
        output_lst_mint: _,
        state,
    } = verify_price_lp_tokens_to_redeem(accounts)?;

    let bytes = state.try_borrow_data()?;
    let state = try_program_state(&bytes)?;

    let result = calculate_price_lp_tokens_to_redeem(state.lp_withdrawal_fee_bps, sol_value)?;
    let result_le = result.to_le_bytes();
    set_return_data(&result_le);

    Ok(())
}

fn verify_price_lp_tokens_to_redeem<'me, 'info>(
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
