use flat_fee_interface::{
    price_exact_in_verify_account_keys, price_exact_in_verify_account_privileges,
    PriceExactInAccounts, PriceExactInIxArgs, PriceExactInKeys,
};
use flat_fee_lib::{
    account_resolvers::PriceExactInFreeArgs, calc::calculate_price_exact_in, utils::try_fee_account,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data,
    program_error::ProgramError,
};

pub fn process_price_exact_in(
    accounts: &[AccountInfo],
    PriceExactInIxArgs { amount, sol_value }: PriceExactInIxArgs,
) -> ProgramResult {
    let checked = verify_price_exact_in(accounts)?;
    process_price_exact_in_unchecked(checked, amount, sol_value)
}

fn process_price_exact_in_unchecked(
    PriceExactInAccounts {
        input_lst_mint: _,
        output_lst_mint: _,
        input_fee_acc,
        output_fee_acc,
    }: PriceExactInAccounts,
    _amount: u64,
    sol_value: u64,
) -> ProgramResult {
    let input_fee_acc_bytes = input_fee_acc.try_borrow_data()?;
    let input_fee_acc = try_fee_account(&input_fee_acc_bytes)?;
    let output_fee_acc_bytes = output_fee_acc.try_borrow_data()?;
    let output_fee_acc = try_fee_account(&output_fee_acc_bytes)?;

    let result = calculate_price_exact_in(
        input_fee_acc.input_fee_bps,
        output_fee_acc.output_fee_bps,
        sol_value,
    )?;
    let result_le = result.to_le_bytes();
    set_return_data(&result_le);
    Ok(())
}

fn verify_price_exact_in<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<PriceExactInAccounts<'me, 'info>, ProgramError> {
    let actual: PriceExactInAccounts = load_accounts(accounts)?;

    let free_args = PriceExactInFreeArgs {
        input_lst_mint: *actual.input_lst_mint.key,
        output_lst_mint: *actual.output_lst_mint.key,
    };
    let expected: PriceExactInKeys = free_args.resolve();

    price_exact_in_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    price_exact_in_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
