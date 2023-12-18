use flat_fee_interface::{
    set_manager_verify_account_keys, set_manager_verify_account_privileges, SetManagerAccounts,
    SetManagerKeys,
};
use flat_fee_lib::{account_resolvers::SetManagerFreeArgs, utils::try_program_state_mut};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_set_manager(accounts: &[AccountInfo]) -> ProgramResult {
    let SetManagerAccounts {
        new_manager, state, ..
    } = verify_set_manager(accounts)?;

    let mut bytes = state.try_borrow_mut_data()?;
    let state = try_program_state_mut(&mut bytes)?;

    state.manager = *new_manager.key;

    Ok(())
}

fn verify_set_manager<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<SetManagerAccounts<'me, 'info>, ProgramError> {
    let actual: SetManagerAccounts = load_accounts(accounts)?;

    let free_args = SetManagerFreeArgs {
        new_manager: *actual.new_manager.key,
        state_acc: actual.state,
    };
    let expected: SetManagerKeys = free_args.resolve()?;

    set_manager_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    set_manager_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
