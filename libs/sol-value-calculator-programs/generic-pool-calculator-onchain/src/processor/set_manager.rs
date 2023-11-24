use generic_pool_calculator_interface::SetManagerAccounts;
use generic_pool_calculator_lib::utils::try_calculator_state_mut;
use solana_program::program_error::ProgramError;

/// Call on resolved and checked SetManagerAccounts
pub fn process_set_manager_unchecked(
    SetManagerAccounts {
        manager: _,
        new_manager,
        state,
    }: SetManagerAccounts,
) -> Result<(), ProgramError> {
    let mut bytes = state.try_borrow_mut_data()?;
    let calc_state = try_calculator_state_mut(&mut bytes)?;
    calc_state.manager = *new_manager.key;
    Ok(())
}
