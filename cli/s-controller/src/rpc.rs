use std::collections::HashMap;

use data_encoding::BASE64;
use rand::Rng;
use s_controller_lib::{find_disable_pool_authority_list_address, find_pool_state_address};
use solana_account_decoder::{UiAccount, UiAccountData, UiAccountEncoding};
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig},
    rpc_response::RpcSimulateTransactionResult,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey, stake,
    transaction::VersionedTransaction,
};

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
                replace_recent_blockhash: true, // must set to true or sim will error with blockhash not found
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

pub async fn fetch_accounts_as_map(
    rpc: &RpcClient,
    accounts_to_fetch: &[Pubkey],
) -> HashMap<Pubkey, Account> {
    // TODO: perform in batches of 5 for free RPCs
    rpc.get_multiple_accounts(accounts_to_fetch)
        .await
        .unwrap()
        .into_iter()
        .zip(accounts_to_fetch)
        .filter_map(|(acc, pk)| acc.map(|acc| (*pk, acc)))
        .collect()
}

// Returns (pubkey, seed)
pub async fn find_unused_stake_prog_create_with_seed(
    rpc: &RpcClient,
    base: &Pubkey,
) -> (Pubkey, String) {
    // MAX_SEED_LEN = 32, just randomly generate u32 as string to make seed
    const MAX_ATTEMPTS: usize = 5;
    let mut rng = rand::thread_rng();
    for _i in 0..MAX_ATTEMPTS {
        let seed: u32 = rng.gen();
        let seed = seed.to_string();
        let pk = Pubkey::create_with_seed(base, &seed, &stake::program::ID).unwrap();
        let acc = rpc
            .get_account_with_commitment(&pk, CommitmentConfig::processed())
            .await
            .unwrap();
        if acc.value.is_none() {
            return (pk, seed);
        }
    }
    panic!("Could not find unused seed for new stake account");
}
