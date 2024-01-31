use s_controller_interface::{
    add_disable_pool_authority_verify_account_keys,
    add_disable_pool_authority_verify_account_privileges, AddDisablePoolAuthorityAccounts,
    SControllerError,
};
use s_controller_lib::{
    program::{DISABLE_POOL_AUTHORITY_LIST_BUMP, DISABLE_POOL_AUTHORITY_LIST_SEED},
    try_disable_pool_authority_list, try_disable_pool_authority_list_mut,
    AddDisablePoolAuthorityFreeArgs,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::list_account::{extend_list_pda, ExtendListPdaAccounts};

pub fn process_add_disable_pool_authority(accounts: &[AccountInfo]) -> ProgramResult {
    let checked = verify_add_disable_pool_authority(accounts)?;

    extend_list_pda::<Pubkey>(
        ExtendListPdaAccounts {
            list_pda: checked.disable_pool_authority_list,
            payer: checked.payer,
        },
        &[&[
            DISABLE_POOL_AUTHORITY_LIST_SEED,
            &[DISABLE_POOL_AUTHORITY_LIST_BUMP],
        ]],
    )?;

    let mut disable_pool_authority_list_data =
        checked.disable_pool_authority_list.try_borrow_mut_data()?;
    let list = try_disable_pool_authority_list_mut(&mut disable_pool_authority_list_data)?;
    let new_entry = list
        .last_mut()
        .ok_or(SControllerError::InvalidDisablePoolAuthorityListData)?;

    *new_entry = *checked.new_authority.key;

    Ok(())
}

fn verify_not_duplicate(
    disable_pool_authority_list: &AccountInfo,
    authority: Pubkey,
) -> Result<(), ProgramError> {
    let d = disable_pool_authority_list.try_borrow_data()?;
    let disable_pool_authority_list = try_disable_pool_authority_list(&d)?;
    if disable_pool_authority_list.contains(&authority) {
        Err(SControllerError::DuplicateDisablePoolAuthority.into())
    } else {
        Ok(())
    }
}

fn verify_add_disable_pool_authority<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<AddDisablePoolAuthorityAccounts<'me, 'info>, ProgramError> {
    let actual: AddDisablePoolAuthorityAccounts = load_accounts(accounts)?;

    let free_args = AddDisablePoolAuthorityFreeArgs {
        payer: *actual.payer.key,
        new_authority: *actual.new_authority.key,
        pool_state_acc: actual.pool_state,
    };
    let expected = free_args.resolve()?;

    add_disable_pool_authority_verify_account_keys(actual, expected)
        .map_err(log_and_return_wrong_acc_err)?;
    add_disable_pool_authority_verify_account_privileges(actual)
        .map_err(log_and_return_acc_privilege_err)?;

    verify_not_duplicate(
        actual.disable_pool_authority_list,
        *actual.new_authority.key,
    )?;

    Ok(actual)
}
