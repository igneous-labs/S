use generic_pool_calculator_interface::{
    set_manager_verify_account_keys, set_manager_verify_account_privileges, SetManagerAccounts,
    SetManagerKeys,
};
use generic_pool_calculator_lib::account_resolvers::SetManagerRootAccounts;
use generic_pool_calculator_onchain::processor::process_set_manager_unchecked;
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_lib::SplSolValCalc;

pub fn process_set_manager(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify(accounts)?;
    process_set_manager_unchecked(checked)
}

fn verify<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<SetManagerAccounts<'me, 'info>, ProgramError> {
    let actual: SetManagerAccounts = load_accounts(accounts)?;

    let root_keys: SetManagerRootAccounts<SplSolValCalc, _> = SetManagerRootAccounts {
        new_manager: *actual.new_manager.key,
        state: actual.state,
        phantom: Default::default(),
    };
    let expected: SetManagerKeys = root_keys.resolve()?;

    set_manager_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    set_manager_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
