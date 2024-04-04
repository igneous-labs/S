use std::convert::Infallible;

use generic_pool_calculator_lib::account_resolvers::LstSolCommonIntermediateKeys;
use lazy_static::lazy_static;
use lido_calculator_lib::lido_sol_val_calc_account_metas;
use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use s_cli_utils::srlut;
use s_controller_interface::PoolState;
use sanctum_lst_list::{PoolInfo, SanctumLst, SanctumLstList, SplPoolAccounts};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    address_lookup_table::{state::AddressLookupTable, AddressLookupTableAccount},
    instruction::AccountMeta,
    pubkey::Pubkey,
};
use spl_calculator_lib::{
    resolve_to_account_metas_for_calc, SanctumSplMultiSolValCalc, SanctumSplSolValCalc,
    SplSolValCalc,
};
use wsol_calculator_lib::WSOL_LST_SOL_COMMON_METAS;

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

/// Returns program ID of the sol value calculator program corresponding to the LST's program
pub fn sol_val_calc_of_sanctum_lst(sanctum_lst: &SanctumLst) -> Pubkey {
    match sanctum_lst.pool {
        PoolInfo::Lido => lido_calculator_lib::program::ID,
        PoolInfo::Marinade => marinade_calculator_lib::program::ID,
        PoolInfo::ReservePool => wsol_calculator_lib::program::ID,
        PoolInfo::SanctumSpl(_) => spl_calculator_lib::sanctum_spl_sol_val_calc_program::ID,
        PoolInfo::Spl(_) => spl_calculator_lib::program::ID,
        PoolInfo::SanctumSplMulti(_) => {
            spl_calculator_lib::sanctum_spl_multi_sol_val_calc_program::ID
        }
        PoolInfo::SPool(_) => todo!(),
    }
}

/// Returns the accounts suffix slice required to call LstToSol or SolToLst, excluding
/// the sol value calculator program ID.
///
/// This can be used as the 2nd arg of [`s_controller_lib::ix_extend_with_sol_value_calculator_accounts`] directly
pub fn sol_value_calculator_accounts_of_sanctum_lst(
    SanctumLst { mint, pool, .. }: &SanctumLst,
) -> Vec<AccountMeta> {
    match pool {
        PoolInfo::Lido => lido_sol_val_calc_account_metas().to_vec(),
        PoolInfo::Marinade => marinade_sol_val_calc_account_metas().to_vec(),
        PoolInfo::ReservePool => WSOL_LST_SOL_COMMON_METAS.to_vec(),
        PoolInfo::SanctumSpl(SplPoolAccounts { pool, .. }) => {
            resolve_to_account_metas_for_calc::<SanctumSplSolValCalc>(
                LstSolCommonIntermediateKeys {
                    lst_mint: *mint,
                    pool_state: *pool,
                },
            )
            .to_vec()
        }
        PoolInfo::Spl(SplPoolAccounts { pool, .. }) => {
            resolve_to_account_metas_for_calc::<SplSolValCalc>(LstSolCommonIntermediateKeys {
                lst_mint: *mint,
                pool_state: *pool,
            })
            .to_vec()
        }
        PoolInfo::SanctumSplMulti(SplPoolAccounts { pool, .. }) => {
            resolve_to_account_metas_for_calc::<SanctumSplMultiSolValCalc>(
                LstSolCommonIntermediateKeys {
                    lst_mint: *mint,
                    pool_state: *pool,
                },
            )
            .to_vec()
        }
        PoolInfo::SPool(_) => todo!(),
    }
}

pub fn find_sanctum_lst_by_mint(mint: Pubkey) -> Option<&'static SanctumLst> {
    SANCTUM_LST_LIST
        .sanctum_lst_list
        .iter()
        .find(|lst| lst.mint == mint)
}

pub async fn fetch_srlut(rpc: &RpcClient) -> AddressLookupTableAccount {
    let srlut = rpc.get_account(&srlut::ID).await.unwrap();
    AddressLookupTableAccount {
        key: srlut::ID,
        addresses: AddressLookupTable::deserialize(&srlut.data)
            .unwrap()
            .addresses
            .into(),
    }
}
