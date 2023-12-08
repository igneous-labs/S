use flat_fee_interface::{
    price_lp_tokens_to_mint_verify_account_keys, price_lp_tokens_to_mint_verify_account_privileges,
    PriceLpTokensToMintAccounts, PriceLpTokensToMintIxArgs, PriceLpTokensToMintKeys,
};
use flat_fee_lib::{
    account_resolvers::PriceLpTokensToMintFreeArgs, calc::calculate_price_lp_tokens_to_mint,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data,
    program_error::ProgramError,
};

pub fn process_price_lp_tokens_to_mint(
    accounts: &[AccountInfo],
    PriceLpTokensToMintIxArgs { sol_value, .. }: PriceLpTokensToMintIxArgs,
) -> ProgramResult {
    verify_price_lp_tokens_to_mint(accounts)?;

    let result = calculate_price_lp_tokens_to_mint(sol_value)?;
    let result_le = result.to_le_bytes();
    set_return_data(&result_le);

    Ok(())
}

fn verify_price_lp_tokens_to_mint<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<PriceLpTokensToMintAccounts<'me, 'info>, ProgramError> {
    let actual: PriceLpTokensToMintAccounts = load_accounts(accounts)?;

    let free_args = PriceLpTokensToMintFreeArgs {
        input_lst_mint: *actual.input_lst_mint.key,
    };
    let expected: PriceLpTokensToMintKeys = free_args.resolve();

    price_lp_tokens_to_mint_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    price_lp_tokens_to_mint_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
