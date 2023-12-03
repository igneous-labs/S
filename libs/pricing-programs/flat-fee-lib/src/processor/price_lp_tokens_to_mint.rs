use flat_fee_interface::{
    price_lp_tokens_to_mint_verify_account_keys, price_lp_tokens_to_mint_verify_account_privileges,
    PriceLpTokensToMintAccounts, PriceLpTokensToMintKeys,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data,
    program_error::ProgramError,
};

use crate::account_resolvers::PriceLpTokensToMintFreeArgs;

pub fn process_price_lp_tokens_to_mint_unchecked(
    PriceLpTokensToMintAccounts { input_lst_mint: _ }: PriceLpTokensToMintAccounts,
    _amount: u64,
    sol_value: u64,
) -> ProgramResult {
    // TODO: calculate the sol value of lp tokens to mint
    let result = sol_value;
    let result_le = result.to_le_bytes();
    set_return_data(&result_le);
    Ok(())
}

pub fn verify_price_lp_tokens_to_mint<'me, 'info>(
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
