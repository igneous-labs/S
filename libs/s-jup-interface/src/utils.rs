use std::sync::{atomic::AtomicU64, Arc};

use s_controller_interface::{LstState, PoolState};
use s_pricing_prog_aggregate::{KnownPricingProg, MutablePricingProg};
use s_sol_val_calc_prog_aggregate::{
    KnownLstSolValCalc, LidoLstSolValCalc, LstSolValCalc, MarinadeLstSolValCalc,
    SanctumSplLstSolValCalc, SanctumSplMultiLstSolValCalc, SplLstSolValCalc,
    SplLstSolValCalcInitKeys, WsolLstSolValCalc,
};
use sanctum_lst_list::{PoolInfo, SanctumLst, SplPoolAccounts};

use crate::LstData;

pub fn try_pricing_prog(
    pool_state: &PoolState,
    lst_state_list: &[LstState],
) -> anyhow::Result<KnownPricingProg> {
    Ok(KnownPricingProg::try_new(
        pool_state.pricing_program,
        lst_state_list.iter().map(|LstState { mint, .. }| *mint),
    )?)
}

pub fn try_lst_data(
    lst_list: &[SanctumLst],
    LstState {
        mint,
        sol_value_calculator,
        ..
    }: &LstState,
    shared_current_epoch: Arc<AtomicU64>,
) -> Option<LstData> {
    let SanctumLst {
        pool,
        token_program,
        ..
    } = lst_list.iter().find(|s| s.mint == *mint)?;
    let calc = match pool {
        PoolInfo::Lido => KnownLstSolValCalc::Lido(LidoLstSolValCalc::new(shared_current_epoch)),
        PoolInfo::Marinade => KnownLstSolValCalc::Marinade(MarinadeLstSolValCalc::default()),
        PoolInfo::ReservePool => KnownLstSolValCalc::Wsol(WsolLstSolValCalc),
        PoolInfo::SanctumSpl(SplPoolAccounts { pool, .. }) => {
            KnownLstSolValCalc::SanctumSpl(SanctumSplLstSolValCalc::from_keys(
                SplLstSolValCalcInitKeys {
                    lst_mint: *mint,
                    stake_pool_addr: *pool,
                },
                shared_current_epoch,
            ))
        }
        PoolInfo::Spl(SplPoolAccounts { pool, .. }) => {
            KnownLstSolValCalc::Spl(SplLstSolValCalc::from_keys(
                SplLstSolValCalcInitKeys {
                    lst_mint: *mint,
                    stake_pool_addr: *pool,
                },
                shared_current_epoch,
            ))
        }
        PoolInfo::SanctumSplMulti(SplPoolAccounts { pool, .. }) => {
            KnownLstSolValCalc::SanctumSplMulti(SanctumSplMultiLstSolValCalc::from_keys(
                SplLstSolValCalcInitKeys {
                    lst_mint: *mint,
                    stake_pool_addr: *pool,
                },
                shared_current_epoch,
            ))
        }
        PoolInfo::SPool(_) => None?,
    };
    if *sol_value_calculator != calc.sol_value_calculator_program_id() {
        None
    } else {
        Some(LstData {
            sol_val_calc: calc,
            reserves_balance: None,
            token_program: *token_program,
        })
    }
}
