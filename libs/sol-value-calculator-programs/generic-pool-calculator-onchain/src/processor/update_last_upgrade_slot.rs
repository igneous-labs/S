use generic_pool_calculator_interface::{
    update_last_upgrade_slot_verify_account_keys,
    update_last_upgrade_slot_verify_account_privileges, UpdateLastUpgradeSlotAccounts,
    UpdateLastUpgradeSlotKeys,
};
use generic_pool_calculator_lib::{
    account_resolvers::UpdateLastUpgradeSlotFreeArgs,
    utils::{read_stake_pool_progdata_meta, try_calculator_state_mut},
    GenericPoolSolValCalc,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

/// Call on resolved and checked UpdateLastUpgradeSlotAccounts
pub fn process_update_last_upgrade_slot_unchecked(
    UpdateLastUpgradeSlotAccounts {
        manager: _,
        state,
        pool_program: _,
        pool_program_data,
    }: UpdateLastUpgradeSlotAccounts,
) -> Result<(), ProgramError> {
    let (last_upgrade_slot, _upgrade_auth) = read_stake_pool_progdata_meta(pool_program_data)?;
    let mut bytes = state.try_borrow_mut_data()?;
    let calc_state = try_calculator_state_mut(&mut bytes)?;
    calc_state.last_upgrade_slot = last_upgrade_slot;
    Ok(())
}

pub fn verify_update_last_upgrade_slot<'me, 'info, P: GenericPoolSolValCalc>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<UpdateLastUpgradeSlotAccounts<'me, 'info>, ProgramError> {
    let actual: UpdateLastUpgradeSlotAccounts = load_accounts(accounts)?;

    let root_keys = UpdateLastUpgradeSlotFreeArgs {
        pool_program: actual.pool_program,
        state: actual.state,
    };
    let expected: UpdateLastUpgradeSlotKeys = root_keys.resolve::<P>()?;

    update_last_upgrade_slot_verify_account_keys(actual, expected)
        .map_err(log_and_return_wrong_acc_err)?;
    update_last_upgrade_slot_verify_account_privileges(actual)
        .map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
