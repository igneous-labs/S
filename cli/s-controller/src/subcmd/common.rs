use std::convert::Infallible;

use s_controller_interface::PoolState;
use s_controller_lib::U8Bool;
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

pub fn verify_disable_pool_authority(
    disable_pool_authority_list: &[Pubkey],
    authority: Pubkey,
) -> Result<(), Infallible> {
    if !disable_pool_authority_list.contains(&authority) {
        eprintln!("Unauthorized authority: {}", authority);
        std::process::exit(-1);
    }
    Ok(())
}

pub fn verify_pool_is_not_rebalancing(pool_state: &PoolState) -> Result<(), Infallible> {
    if U8Bool(pool_state.is_rebalancing).is_true() {
        eprintln!("Could not execute due to rebalancing state.",);
        std::process::exit(-1);
    }
    Ok(())
}
