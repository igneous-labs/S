use std::convert::Infallible;

use s_controller_interface::PoolState;
use solana_sdk::pubkey::Pubkey;

pub fn verify_admin(state: &PoolState, admin: Pubkey) -> Result<(), Infallible> {
    if state.admin != admin {
        eprintln!("Wrong admin. Expected: {}. Got: {}", state.admin, admin);
        std::process::exit(-1);
    }
    Ok(())
}