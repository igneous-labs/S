use flat_fee_interface::{
    price_exact_out_verify_account_keys, price_exact_out_verify_account_privileges,
    PriceExactOutAccounts, PriceExactOutIxArgs, PriceExactOutKeys,
};
use flat_fee_lib::{
    account_resolvers::{PriceExactOutFreeArgs, PriceExactOutWithBumpFreeArgs},
    calc::{calculate_price_exact_out, CalculatePriceExactOut},
    utils::try_fee_account,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data,
    program_error::ProgramError,
};

pub fn process_price_exact_out(
    accounts: &[AccountInfo],
    PriceExactOutIxArgs {
        amount: _,
        sol_value,
    }: PriceExactOutIxArgs,
) -> ProgramResult {
    let PriceExactOutAccounts {
        input_lst_mint: _,
        output_lst_mint: _,
        input_fee_acc,
        output_fee_acc,
    } = verify_price_exact_out(accounts)?;

    let input_fee_acc_bytes = input_fee_acc.try_borrow_data()?;
    let input_fee_acc = try_fee_account(&input_fee_acc_bytes)?;
    let output_fee_acc_bytes = output_fee_acc.try_borrow_data()?;
    let output_fee_acc = try_fee_account(&output_fee_acc_bytes)?;

    let result = calculate_price_exact_out(CalculatePriceExactOut {
        input_fee_bps: input_fee_acc.input_fee_bps,
        output_fee_bps: output_fee_acc.output_fee_bps,
        sol_value,
    })?;
    let result_le = result.to_le_bytes();
    set_return_data(&result_le);

    Ok(())
}

fn verify_price_exact_out<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<PriceExactOutAccounts<'me, 'info>, ProgramError> {
    let actual: PriceExactOutAccounts = load_accounts(accounts)?;

    let input_fee_acc_bytes = actual.input_fee_acc.try_borrow_data()?;
    let input_fee_acc_bump = try_fee_account(&input_fee_acc_bytes)?.bump;
    let output_fee_acc_bytes = actual.output_fee_acc.try_borrow_data()?;
    let output_fee_acc_bump = try_fee_account(&output_fee_acc_bytes)?.bump;

    let free_args = PriceExactOutWithBumpFreeArgs {
        find_pda_args: PriceExactOutFreeArgs {
            input_lst_mint: *actual.input_lst_mint.key,
            output_lst_mint: *actual.output_lst_mint.key,
        },
        input_fee_acc_bump,
        output_fee_acc_bump,
    };
    let expected: PriceExactOutKeys = free_args.resolve()?;

    price_exact_out_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    price_exact_out_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
