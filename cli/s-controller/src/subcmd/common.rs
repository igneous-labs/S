use std::convert::Infallible;

use s_controller_interface::PoolState;
use solana_sdk::pubkey::Pubkey;

pub fn verify_admin(pool_state: &PoolState, curr_admin: Pubkey) -> Result<(), Infallible> {
    if pool_state.admin != curr_admin {
        eprintln!(
            "Wrong admin. Expected: {}. Got: {}",
            pool_state.admin, curr_admin
        );
        std::process::exit(-1);
    }
    Ok(())
}
