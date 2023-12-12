use s_controller_interface::{
    set_rebalance_authority_verify_account_keys, set_rebalance_authority_verify_account_privileges,
    SControllerError, SetRebalanceAuthorityAccounts,
};
use s_controller_lib::{try_pool_state, try_pool_state_mut, SetRebalanceAuthorityFreeArgs};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::verify::verify_not_rebalancing_and_not_disabled;

pub fn process_set_rebalance_authority(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts = verify_set_rebalance_authority(accounts)?;

    let mut pool_state_bytes = accounts.pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_bytes)?;

    pool_state.rebalance_authority = *accounts.new_rebalance_authority.key;

    Ok(())
}

fn verify_set_rebalance_authority<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
) -> Result<SetRebalanceAuthorityAccounts<'a, 'info>, ProgramError> {
    let actual: SetRebalanceAuthorityAccounts = load_accounts(accounts)?;

    let expected = SetRebalanceAuthorityFreeArgs {
        signer: *actual.signer.key,
        new_rebalance_authority: *actual.new_rebalance_authority.key,
    }
    .resolve();

    set_rebalance_authority_verify_account_keys(actual, expected)
        .map_err(log_and_return_wrong_acc_err)?;
    set_rebalance_authority_verify_account_privileges(actual)
        .map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    if *actual.signer.key != pool_state.admin
        && *actual.signer.key != pool_state.rebalance_authority
    {
        return Err(SControllerError::UnauthorizedSetRebalanceAuthoritySigner.into());
    }

    Ok(actual)
}
