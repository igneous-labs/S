use flat_fee_interface::{
    set_lst_fee_verify_account_keys, set_lst_fee_verify_account_privileges, SetLstFeeAccounts,
    SetLstFeeIxArgs, SetLstFeeKeys,
};
use flat_fee_lib::{account_resolvers::SetLstFeeFreeArgs, utils::try_fee_account_mut};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_set_lst_fee(
    accounts: &[AccountInfo],
    SetLstFeeIxArgs {
        input_fee_bps,
        output_fee_bps,
    }: SetLstFeeIxArgs,
) -> ProgramResult {
    let SetLstFeeAccounts {
        manager: _,
        fee_acc,
        state: _,
    } = verify_set_lst_fee(accounts)?;

    let mut bytes = fee_acc.try_borrow_mut_data()?;
    let fee_acc = try_fee_account_mut(&mut bytes)?;

    fee_acc.input_fee_bps = input_fee_bps;
    fee_acc.output_fee_bps = output_fee_bps;

    Ok(())
}

fn verify_set_lst_fee<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<SetLstFeeAccounts<'me, 'info>, ProgramError> {
    let actual: SetLstFeeAccounts = load_accounts(accounts)?;

    let free_args = SetLstFeeFreeArgs {
        state: actual.state,
        fee_acc: *actual.fee_acc.key,
    };
    let expected: SetLstFeeKeys = free_args.resolve()?;

    set_lst_fee_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    set_lst_fee_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
