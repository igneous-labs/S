use s_controller_interface::{
    disable_pool_verify_account_keys, disable_pool_verify_account_privileges, DisablePoolAccounts,
};
use s_controller_lib::{try_pool_state, try_pool_state_mut, DisablePoolFreeArgs, U8BoolMut};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::verify::{
    verify_admin_or_disable_pool_authority, verify_not_rebalancing_and_not_disabled,
};

pub fn process_disable_pool(accounts: &[AccountInfo]) -> ProgramResult {
    let DisablePoolAccounts {
        signer: _,
        pool_state,
        disable_pool_authority_list: _,
    } = verify_disable_pool(accounts)?;

    let mut pool_state_bytes = pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_bytes)?;

    U8BoolMut(&mut pool_state.is_disabled).set_true();

    Ok(())
}

fn verify_disable_pool<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<DisablePoolAccounts<'me, 'info>, ProgramError> {
    let actual: DisablePoolAccounts = load_accounts(accounts)?;

    let free_args = DisablePoolFreeArgs {
        signer: *actual.signer.key,
    };
    let expected = free_args.resolve();

    disable_pool_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    disable_pool_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;

    verify_not_rebalancing_and_not_disabled(pool_state)?;

    // signer should be either admin or disable pool authority
    verify_admin_or_disable_pool_authority(
        *actual.signer.key,
        pool_state,
        actual.disable_pool_authority_list,
    )?;

    Ok(actual)
}
