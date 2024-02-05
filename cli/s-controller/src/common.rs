use std::convert::Infallible;

use lazy_static::lazy_static;
use s_controller_interface::PoolState;
use sanctum_lst_list::{PoolInfo, SanctumLst, SanctumLstList};
use solana_sdk::pubkey::Pubkey;

lazy_static! {
    pub static ref SANCTUM_LST_LIST: SanctumLstList = SanctumLstList::load();
}

pub fn verify_admin(state: &PoolState, admin: Pubkey) -> Result<(), Infallible> {
    if state.admin != admin {
        eprintln!("Wrong admin. Expected: {}. Got: {}", state.admin, admin);
        std::process::exit(-1);
    }
    Ok(())
}

pub fn find_sanctum_lst(mint: Pubkey) -> Option<&'static SanctumLst> {
    SANCTUM_LST_LIST
        .sanctum_lst_list
        .iter()
        .find(|lst| lst.mint == mint)
}

/// Returns program ID of the sol value calculator program corresponding to the LST's program
pub fn sol_val_calc_of_sanctum_lst(sanctum_lst: &SanctumLst) -> Pubkey {
    match sanctum_lst.pool {
        PoolInfo::Lido => lido_calculator_lib::program::ID,
        PoolInfo::Marinade => marinade_calculator_lib::program::ID,
        PoolInfo::ReservePool => wsol_calculator_lib::program::ID,
        PoolInfo::SanctumSpl(_) => {
            sanctum_spl_stake_pool_keys::sanctum_spl_sol_val_calc_program::ID
        }
        PoolInfo::Spl(_) => spl_calculator_lib::program::ID,
        PoolInfo::Socean(_) => panic!("Socean sol val calc todo"),
    }
}
