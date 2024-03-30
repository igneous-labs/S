use std::convert::Infallible;

use borsh::BorshDeserialize;
use data_encoding::BASE64;
use generic_pool_calculator_interface::CalculatorState;
use lido_calculator_lib::lido_sol_val_calc_account_metas;
use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use sanctum_token_ratio::U64ValueRange;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_readonly_account::keyed::Keyed;
use solana_rpc_client_api::response::RpcSimulateTransactionResult;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::{v0::Message, VersionedMessage},
    native_token::lamports_to_sol,
    pubkey::Pubkey,
    signer::Signer,
    transaction::VersionedTransaction,
};
use solana_transaction_status::{UiReturnDataEncoding, UiTransactionReturnData};
use spl_calculator_lib::SplLstSolCommonFreeArgsConst;
use wsol_calculator_lib::WSOL_LST_SOL_COMMON_METAS;

use crate::sol_val_calc_arg::SolValCalcArg;

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

pub async fn lst_sol_common_account_metas(
    rpc: &RpcClient,
    arg: &SolValCalcArg,
    pool: Option<Pubkey>,
) -> Vec<AccountMeta> {
    let pool = match arg {
        SolValCalcArg::SanctumSpl | SolValCalcArg::Spl => {
            pool.expect("pool pubkey must be provided if spl or sanctum-spl")
        }
        _ => Pubkey::default(), // dont care
    };
    match arg {
        SolValCalcArg::Lido => lido_sol_val_calc_account_metas().to_vec(),
        SolValCalcArg::Marinade => marinade_sol_val_calc_account_metas().to_vec(),
        SolValCalcArg::SanctumSpl => {
            let pool_acc = rpc.get_account(&pool).await.unwrap();
            SplLstSolCommonFreeArgsConst {
                spl_stake_pool: Keyed {
                    account: pool_acc,
                    pubkey: pool,
                },
            }
            .resolve_sanctum_spl_to_account_metas()
            .unwrap()
            .to_vec()
        }
        SolValCalcArg::SanctumSplMulti => {
            let pool_acc = rpc.get_account(&pool).await.unwrap();
            SplLstSolCommonFreeArgsConst {
                spl_stake_pool: Keyed {
                    account: pool_acc,
                    pubkey: pool,
                },
            }
            .resolve_sanctum_spl_multi_to_account_metas()
            .unwrap()
            .to_vec()
        }
        SolValCalcArg::Spl => {
            let pool_acc = rpc.get_account(&pool).await.unwrap();
            SplLstSolCommonFreeArgsConst {
                spl_stake_pool: Keyed {
                    account: pool_acc,
                    pubkey: pool,
                },
            }
            .resolve_spl_to_account_metas()
            .unwrap()
            .to_vec()
        }
        SolValCalcArg::Wsol => WSOL_LST_SOL_COMMON_METAS.to_vec(),
        SolValCalcArg::Unknown(_) => unreachable!(),
    }
}

pub async fn handle_lst_sol_ix(rpc: &RpcClient, ix: Instruction, payer: &dyn Signer) {
    let rbh = rpc.get_latest_blockhash().await.unwrap();
    let tx = VersionedTransaction::try_new(
        VersionedMessage::V0(Message::try_compile(&payer.pubkey(), &[ix], &[], rbh).unwrap()),
        &[payer],
    )
    .unwrap();
    let RpcSimulateTransactionResult {
        return_data,
        err,
        logs,
        ..
    } = rpc.simulate_transaction(&tx).await.unwrap().value;
    if let Some(e) = err {
        eprintln!("Logs:");
        eprintln!("{logs:#?}");
        eprintln!("Err: {e}");
        return;
    }
    let UiTransactionReturnData {
        data: (data_str, encoding),
        ..
    } = return_data.unwrap();
    // Base64 is the only variant rn, but ig rpc might change in the future
    if encoding != UiReturnDataEncoding::Base64 {
        eprintln!(
            "Can only handle base64 encoded return data, cannot handle {encoding:?} encoding"
        );
        return;
    }
    let data = BASE64.decode(data_str.as_bytes()).unwrap();
    let range = U64ValueRange::deserialize(&mut data.as_ref()).unwrap();
    println!(
        "{},{}",
        lamports_to_sol(range.get_min()),
        lamports_to_sol(range.get_max())
    );
}
