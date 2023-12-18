use s_controller_interface::{
    enable_pool_verify_account_keys, enable_pool_verify_account_privileges, EnablePoolAccounts,
    SControllerError,
};
use s_controller_lib::{try_pool_state, try_pool_state_mut, EnablePoolFreeArgs, U8Bool, U8BoolMut};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_enable_pool(accounts: &[AccountInfo]) -> ProgramResult {
    let EnablePoolAccounts {
        admin: _,
        pool_state,
    } = verify_enable_pool(accounts)?;

    let mut pool_state_bytes = pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_bytes)?;

    U8BoolMut(&mut pool_state.is_disabled).set_false();

    Ok(())
}

fn verify_enable_pool<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<EnablePoolAccounts<'me, 'info>, ProgramError> {
    let actual: EnablePoolAccounts = load_accounts(accounts)?;

    let free_args = EnablePoolFreeArgs {
        pool_state_acc: actual.pool_state,
    };
    let expected = free_args.resolve()?;

    enable_pool_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    enable_pool_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_data = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_data)?;

    if U8Bool(pool_state.is_disabled).is_false() {
        return Err(SControllerError::PoolEnabled.into());
    }

    Ok(actual)
}
