use s_controller_interface::{
    add_disable_pool_authority_verify_account_keys,
    add_disable_pool_authority_verify_account_privileges, AddDisablePoolAuthorityAccounts,
};
use s_controller_lib::AddDisablePoolAuthorityFreeArgs;
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_add_disable_pool_authority(accounts: &[AccountInfo]) -> ProgramResult {
    let _checked = verify_add_disable_pool_authority(accounts)?;
    todo!()
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

    add_disable_pool_authority_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    add_disable_pool_authority_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
