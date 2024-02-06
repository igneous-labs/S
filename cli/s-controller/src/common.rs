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

pub fn verify_protocol_fee_beneficiary(
    state: &PoolState,
    beneficiary: Pubkey,
) -> Result<(), Infallible> {
    if state.protocol_fee_beneficiary != beneficiary {
        eprintln!(
            "Wrong beneficiary. Expected: {}. Got: {}",
            state.protocol_fee_beneficiary, beneficiary
        );
        std::process::exit(-1);
    }
    Ok(())
}
