use generic_pool_calculator_interface::{
    update_last_upgrade_slot_verify_account_keys,
    update_last_upgrade_slot_verify_account_privileges, UpdateLastUpgradeSlotAccounts,
    UpdateLastUpgradeSlotKeys,
};
use generic_pool_calculator_lib::account_resolvers::UpdateLastUpgradeSlotRootAccounts;
use generic_pool_calculator_onchain::processor::process_update_last_upgrade_slot_unchecked;
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_lib::SplSolValCalc;

pub fn process_update_last_upgrade_slot(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify(accounts)?;
    process_update_last_upgrade_slot_unchecked(checked)
}

fn verify<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<UpdateLastUpgradeSlotAccounts<'me, 'info>, ProgramError> {
    let actual: UpdateLastUpgradeSlotAccounts = load_accounts(accounts)?;

    let root_keys: UpdateLastUpgradeSlotRootAccounts<SplSolValCalc, _, _> =
        UpdateLastUpgradeSlotRootAccounts {
            pool_program: actual.pool_program,
            state: actual.state,
            phantom: Default::default(),
        };
    let expected: UpdateLastUpgradeSlotKeys = root_keys.resolve()?;

    update_last_upgrade_slot_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    update_last_upgrade_slot_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
