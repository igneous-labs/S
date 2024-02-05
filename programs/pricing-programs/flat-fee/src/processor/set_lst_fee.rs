use flat_fee_interface::{
    set_lst_fee_verify_account_keys, set_lst_fee_verify_account_privileges, SetLstFeeAccounts,
    SetLstFeeIxArgs, SetLstFeeKeys,
};
use flat_fee_lib::{
    account_resolvers::SetLstFeeFreeArgs, fee_bound::verify_signed_fee_bps_bound,
    utils::try_fee_account_mut,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_set_lst_fee(accounts: &[AccountInfo], args: SetLstFeeIxArgs) -> ProgramResult {
    let SetLstFeeAccounts { fee_acc, .. } = verify_set_lst_fee(accounts, &args)?;

    let mut bytes = fee_acc.try_borrow_mut_data()?;
    let fee_acc = try_fee_account_mut(&mut bytes)?;

    fee_acc.input_fee_bps = args.input_fee_bps;
    fee_acc.output_fee_bps = args.output_fee_bps;

    Ok(())
}

fn verify_set_lst_fee<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
    SetLstFeeIxArgs {
        input_fee_bps,
        output_fee_bps,
    }: &SetLstFeeIxArgs,
) -> Result<SetLstFeeAccounts<'me, 'info>, ProgramError> {
    let actual: SetLstFeeAccounts = load_accounts(accounts)?;

    let free_args = SetLstFeeFreeArgs {
        state_acc: actual.state,
        fee_acc: *actual.fee_acc.key,
    };
    let expected: SetLstFeeKeys = free_args.resolve()?;

    set_lst_fee_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    set_lst_fee_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    verify_signed_fee_bps_bound(*input_fee_bps)?;
    verify_signed_fee_bps_bound(*output_fee_bps)?;

    Ok(actual)
}
