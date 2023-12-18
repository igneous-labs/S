use s_controller_interface::{
    remove_disable_pool_authority_verify_account_keys,
    remove_disable_pool_authority_verify_account_privileges, RemoveDisablePoolAuthorityAccounts,
    RemoveDisablePoolAuthorityIxArgs, SControllerError,
};
use s_controller_lib::{index_to_usize, try_pool_state, RemoveDisablePoolAuthorityFreeArgs};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    list_account::{remove_from_list_pda, RemoveFromListPdaAccounts},
    verify::verify_admin_or_disable_pool_authority,
};

pub fn process_remove_disable_pool_authority(
    accounts: &[AccountInfo],
    args: RemoveDisablePoolAuthorityIxArgs,
) -> ProgramResult {
    let (checked_accounts, index) = verify_remove_disable_pool_authority(accounts, args)?;

    remove_from_list_pda::<Pubkey>(
        RemoveFromListPdaAccounts {
            list_pda: checked_accounts.disable_pool_authority_list,
            refund_rent_to: checked_accounts.refund_rent_to,
        },
        index,
    )
}

fn verify_remove_disable_pool_authority<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
    RemoveDisablePoolAuthorityIxArgs { index }: RemoveDisablePoolAuthorityIxArgs,
) -> Result<(RemoveDisablePoolAuthorityAccounts<'me, 'info>, usize), ProgramError> {
    let actual: RemoveDisablePoolAuthorityAccounts = load_accounts(accounts)?;
    let index = index_to_usize(index)?;

    let free_args = RemoveDisablePoolAuthorityFreeArgs {
        index,
        refund_rent_to: *actual.refund_rent_to.key,
        signer: *actual.signer.key,
        pool_state_acc: actual.pool_state,
        disable_pool_authority_list: actual.disable_pool_authority_list,
    };
    let expected = free_args.resolve()?;

    remove_disable_pool_authority_verify_account_keys(actual, expected)
        .map_err(log_and_return_wrong_acc_err)?;
    remove_disable_pool_authority_verify_account_privileges(actual)
        .map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;

    // signer should be either admin or disable pool authority that matches the target authority being removed
    verify_admin_or_disable_pool_authority(
        *actual.signer.key,
        pool_state,
        actual.disable_pool_authority_list,
    )?;
    if *actual.signer.key != pool_state.admin && *actual.signer.key != *actual.authority.key {
        return Err(SControllerError::UnauthorizedDisablePoolAuthoritySigner.into());
    }

    Ok((actual, index))
}
