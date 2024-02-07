use data_encoding::BASE64;
use s_controller_lib::{find_disable_pool_authority_list_address, find_pool_state_address};
use solana_account_decoder::{UiAccount, UiAccountData, UiAccountEncoding};
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig},
    rpc_response::RpcSimulateTransactionResult,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use solana_sdk::{account::Account, pubkey::Pubkey, transaction::VersionedTransaction};

pub async fn fetch_pool_state(rpc: &RpcClient, program_id: Pubkey) -> Account {
    rpc.get_account(&find_pool_state_address(program_id).0)
        .await
        .unwrap()
}

pub async fn fetch_disable_pool_authority_list(rpc: &RpcClient, program_id: Pubkey) -> Account {
    rpc.get_account(&find_disable_pool_authority_list_address(program_id).0)
        .await
        .unwrap()
}

// NB: this fn is currently not tested because our current BanksRpcServer setup doesn't really
// allow simulation with post tx accounts results:
// https://github.com/igneous-labs/sanctum-solana-utils/issues/40#issuecomment-1932036297
// All cmds that rely on this should provide some way to do without it so that they can be tested
pub async fn does_tx_modify_pool_state<P: ReadonlyAccountData + ReadonlyAccountPubkey>(
    rpc: &RpcClient,
    tx: &VersionedTransaction,
    pool_state_before: P,
) -> bool {
    let RpcSimulateTransactionResult {
        err,
        logs,
        accounts,
        ..
    } = rpc
        .simulate_transaction_with_config(
            tx,
            RpcSimulateTransactionConfig {
                sig_verify: false,
                replace_recent_blockhash: false,
                commitment: None,
                encoding: None,
                accounts: Some(RpcSimulateTransactionAccountsConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    addresses: vec![pool_state_before.pubkey().to_string()],
                }),
                min_context_slot: None,
            },
        )
        .await
        .unwrap()
        .value;
    if let Some(err) = err {
        eprintln!("Simulation error: {err}");
        match logs {
            Some(logs) => {
                eprintln!("Logs:");
                eprintln!("{logs:#?}");
            }
            None => eprintln!("No logs available"),
        }
        std::process::exit(-1);
    }
    let UiAccount { data, .. } = accounts.as_ref().unwrap()[0].as_ref().unwrap();
    let pool_state_data_after = match data {
        UiAccountData::Binary(b64_str, encoding) => {
            if *encoding != UiAccountEncoding::Base64 {
                panic!("Unexpected ui account encoding {encoding:?}");
            }
            BASE64.decode(b64_str.as_bytes()).unwrap()
        }
        _ => panic!("Unexpected ui account data {data:?}"),
    };
    let d = pool_state_before.data();
    *pool_state_data_after.as_slice() != **d
}
