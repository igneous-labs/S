use flat_fee_interface::{
    set_lp_withdrawal_fee_verify_account_keys, set_lp_withdrawal_fee_verify_account_privileges,
    SetLpWithdrawalFeeAccounts, SetLpWithdrawalFeeIxArgs, SetLpWithdrawalFeeKeys,
};
use flat_fee_lib::{account_resolvers::SetLpWithdrawalFeeFreeArgs, utils::try_program_state_mut};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_set_lp_withdrawal_fee(
    accounts: &[AccountInfo],
    SetLpWithdrawalFeeIxArgs {
        lp_withdrawal_fee_bps,
    }: SetLpWithdrawalFeeIxArgs,
) -> ProgramResult {
    let checked = verify_set_lp_withdrawal_fee(accounts)?;
    process_set_lp_withdrawal_fee_unchecked(checked, lp_withdrawal_fee_bps)
}

fn process_set_lp_withdrawal_fee_unchecked(
    SetLpWithdrawalFeeAccounts { manager: _, state }: SetLpWithdrawalFeeAccounts,
    lp_withdrawal_fee_bps: u16,
) -> ProgramResult {
    let mut bytes = state.try_borrow_mut_data()?;
    let state = try_program_state_mut(&mut bytes)?;
    state.lp_withdrawal_fee_bps = lp_withdrawal_fee_bps;
    Ok(())
}

fn verify_set_lp_withdrawal_fee<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<SetLpWithdrawalFeeAccounts<'me, 'info>, ProgramError> {
    let actual: SetLpWithdrawalFeeAccounts = load_accounts(accounts)?;

    let free_args = SetLpWithdrawalFeeFreeArgs {
        manager: *actual.manager.key,
    };
    let expected: SetLpWithdrawalFeeKeys = free_args.resolve();

    set_lp_withdrawal_fee_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    set_lp_withdrawal_fee_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
