use generic_pool_calculator_interface::UpdateLastUpgradeSlotAccounts;
use generic_pool_calculator_lib::utils::{read_stake_pool_progdata_meta, try_calculator_state_mut};
use solana_program::program_error::ProgramError;

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
