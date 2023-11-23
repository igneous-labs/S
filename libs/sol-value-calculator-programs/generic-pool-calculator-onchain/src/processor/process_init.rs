use bytemuck::try_from_bytes_mut;
use generic_pool_calculator_interface::{CalculatorState, InitAccounts};
use generic_pool_calculator_lib::{GenericPoolSolValCalc, CALCULATOR_STATE_SEED};
use sanctum_onchain_utils::system_program::{create_pda, CreateAccountAccounts, CreateAccountArgs};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

/// Call on resolved and checked InitAccounts
pub fn process_init<P: GenericPoolSolValCalc>(
    InitAccounts {
        payer,
        state,
        system_program: _,
    }: InitAccounts,
    initial_manager: Pubkey,
) -> Result<(), ProgramError> {
    let space = std::mem::size_of::<CalculatorState>();
    create_pda(
        CreateAccountAccounts {
            from: payer,
            to: state,
        },
        CreateAccountArgs {
            space,
            owner: P::ID,
        },
        &[&[CALCULATOR_STATE_SEED, &[P::CALCULATOR_STATE_BUMP]]],
    )?;

    let mut bytes = state.try_borrow_mut_data()?;
    let calc_state: &mut CalculatorState =
        try_from_bytes_mut(&mut bytes).map_err(|_e| ProgramError::AccountDataTooSmall)?;

    calc_state.manager = initial_manager;
    calc_state.last_upgrade_slot = 0;
    Ok(())
}
