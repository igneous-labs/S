//! SyncSolValue stuff is used across multiple instruction processors

use s_controller_interface::{
    sync_sol_value_verify_account_keys, sync_sol_value_verify_account_privileges,
    SyncSolValueAccounts,
};
use s_controller_lib::{try_pool_state, SyncSolValueFreeAccounts};
use sanctum_onchain_utils::utils::{
    log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::program_error::ProgramError;

use super::verify_not_rebalancing_and_not_disabled;

/// exported for use by other instruction processors
pub fn verify_sync_sol_value_accounts<'a, 'info, I: TryInto<usize>>(
    actual: SyncSolValueAccounts<'a, 'info>,
    lst_index: I,
) -> Result<SyncSolValueAccounts<'a, 'info>, ProgramError> {
    let free_accounts = SyncSolValueFreeAccounts {
        lst_index,
        lst_state_list: actual.lst_state_list,
        lst_mint: actual.lst,
    };
    let expected = free_accounts.resolve()?;

    sync_sol_value_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    sync_sol_value_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;

    verify_not_rebalancing_and_not_disabled(pool_state)?;

    Ok(actual)
}
