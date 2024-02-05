use flat_fee_interface::{
    set_lp_withdrawal_fee_verify_account_keys, set_lp_withdrawal_fee_verify_account_privileges,
    SetLpWithdrawalFeeAccounts, SetLpWithdrawalFeeIxArgs, SetLpWithdrawalFeeKeys,
};
use flat_fee_lib::{
    account_resolvers::SetLpWithdrawalFeeFreeArgs, fee_bound::verify_unsigned_fee_bps_bound,
    utils::try_program_state_mut,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_set_lp_withdrawal_fee(
    accounts: &[AccountInfo],
    args: SetLpWithdrawalFeeIxArgs,
) -> ProgramResult {
    let SetLpWithdrawalFeeAccounts { state, .. } = verify_set_lp_withdrawal_fee(accounts, &args)?;

    let mut bytes = state.try_borrow_mut_data()?;
    let state = try_program_state_mut(&mut bytes)?;
    state.lp_withdrawal_fee_bps = args.lp_withdrawal_fee_bps;

    Ok(())
}

fn verify_set_lp_withdrawal_fee<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
    SetLpWithdrawalFeeIxArgs {
        lp_withdrawal_fee_bps,
    }: &SetLpWithdrawalFeeIxArgs,
) -> Result<SetLpWithdrawalFeeAccounts<'me, 'info>, ProgramError> {
    let actual: SetLpWithdrawalFeeAccounts = load_accounts(accounts)?;

    let free_args = SetLpWithdrawalFeeFreeArgs {
        state_acc: actual.state,
    };
    let expected: SetLpWithdrawalFeeKeys = free_args.resolve()?;

    set_lp_withdrawal_fee_verify_account_keys(actual, expected)
        .map_err(log_and_return_wrong_acc_err)?;
    set_lp_withdrawal_fee_verify_account_privileges(actual)
        .map_err(log_and_return_acc_privilege_err)?;

    verify_unsigned_fee_bps_bound(*lp_withdrawal_fee_bps)?;

    Ok(actual)
}
