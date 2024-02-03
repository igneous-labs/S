use std::convert::Infallible;

use flat_fee_interface::ProgramState;
use solana_sdk::pubkey::Pubkey;

pub fn verify_manager(state: &ProgramState, curr_manager: Pubkey) -> Result<(), Infallible> {
    if state.manager != curr_manager {
        eprintln!(
            "Wrong manager. Expected: {}. Got: {}",
            state.manager, curr_manager
        );
        std::process::exit(-1);
    }
    Ok(())
}
