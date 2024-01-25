use std::convert::Infallible;

use generic_pool_calculator_interface::CalculatorState;
use solana_sdk::pubkey::Pubkey;

pub fn verify_manager(state: &CalculatorState, curr_manager: Pubkey) -> Result<(), Infallible> {
    if state.manager != curr_manager {
        eprintln!(
            "Wrong manager. Expected: {}. Got: {}",
            state.manager, curr_manager
        );
        std::process::exit(-1);
    }
    Ok(())
}
