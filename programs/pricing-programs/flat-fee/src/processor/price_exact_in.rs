use flat_fee_interface::{
    price_exact_in_verify_account_keys, PriceExactInAccounts, PriceExactInIxArgs, PriceExactInKeys,
};
use flat_fee_lib::{
    account_resolvers::{PriceExactInFreeArgs, PriceExactInWithBumpFreeArgs},
    calc::{calculate_price_exact_in, CalculatePriceExactInArgs},
    utils::try_fee_account,
};
use sanctum_misc_utils::{load_accounts, log_and_return_wrong_acc_err};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data,
    program_error::ProgramError,
};

pub fn process_price_exact_in(
    accounts: &[AccountInfo],
    PriceExactInIxArgs { sol_value, .. }: PriceExactInIxArgs,
) -> ProgramResult {
    let PriceExactInAccounts {
        input_fee_acc,
        output_fee_acc,
        ..
    } = verify_price_exact_in(accounts)?;

    let input_fee_acc_bytes = input_fee_acc.try_borrow_data()?;
    let input_fee_acc = try_fee_account(&input_fee_acc_bytes)?;
    let output_fee_acc_bytes = output_fee_acc.try_borrow_data()?;
    let output_fee_acc = try_fee_account(&output_fee_acc_bytes)?;

    let result = calculate_price_exact_in(CalculatePriceExactInArgs {
        input_fee_bps: input_fee_acc.input_fee_bps,
        output_fee_bps: output_fee_acc.output_fee_bps,
        in_sol_value: sol_value,
    })?;
    let result_le = result.to_le_bytes();
    set_return_data(&result_le);

    Ok(())
}

fn verify_price_exact_in<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<PriceExactInAccounts<'me, 'info>, ProgramError> {
    let actual: PriceExactInAccounts = load_accounts(accounts)?;

    let input_fee_acc_bytes = actual.input_fee_acc.try_borrow_data()?;
    let input_fee_acc_bump = try_fee_account(&input_fee_acc_bytes)?.bump;
    let output_fee_acc_bytes = actual.output_fee_acc.try_borrow_data()?;
    let output_fee_acc_bump = try_fee_account(&output_fee_acc_bytes)?.bump;

    let free_args = PriceExactInWithBumpFreeArgs {
        args: PriceExactInFreeArgs {
            input_lst_mint: *actual.input_lst_mint.key,
            output_lst_mint: *actual.output_lst_mint.key,
        },
        input_fee_acc_bump,
        output_fee_acc_bump,
    };
    let expected: PriceExactInKeys = free_args.resolve()?;

    price_exact_in_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;

    Ok(actual)
}
