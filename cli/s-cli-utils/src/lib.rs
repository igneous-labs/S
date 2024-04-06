use sanctum_solana_cli_utils::{
    HandleTxArgs, RecentBlockhash, TxSendMode, TxSendingNonblockingRpcClient,
};
use sanctum_solana_client_utils::{
    get_compute_budget_ixs_auto_nonblocking, ComputeBudgetFeeLimit, SortedSigners,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::{address_lookup_table::AddressLookupTableAccount, instruction::Instruction};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    signer::Signer,
    transaction::VersionedTransaction,
};

pub const CONFIG_HELP: &str =
    "Path to solana CLI config. Defaults to solana cli default if not provided";

pub const TX_SEND_MODE_HELP: &str = "Transaction send mode.
- send-actual: signs and sends the tx to the cluster specified in config and outputs hash to stderr
- sim-only: simulates the tx against the cluster and outputs logs to stderr
- dump-msg: dumps the base64 encoded tx to stdout. For use with inspectors and multisigs
";

pub const FEE_LIMIT_CB_HELP: &str = "Max priority fee to pay, in lamports";

pub mod srlut {
    sanctum_macros::declare_program_keys!("KtrvWWkPkhSWM9VMqafZhgnTuozQiHzrBDT8oPcMj3T", []);
}

pub const CU_BUFFER_RATIO: f64 = 1.125;

/// First signer in signers is transaction payer
pub async fn handle_tx_full(
    rpc: &RpcClient,
    fee_limit_lamports: u64,
    send_mode: TxSendMode,
    mut ixs: Vec<Instruction>,
    luts: &[AddressLookupTableAccount],
    signers: &mut [&dyn Signer],
) {
    let payer_pk = signers[0].pubkey();
    signers.sort_by_key(|s| s.pubkey());
    let ixs = match send_mode {
        TxSendMode::DumpMsg => ixs,
        _ => {
            let cb_ixs = get_compute_budget_ixs_auto_nonblocking(
                rpc,
                &payer_pk,
                &ixs,
                luts,
                &ComputeBudgetFeeLimit::TotalLamports(fee_limit_lamports),
                CU_BUFFER_RATIO,
            )
            .await
            .unwrap();
            for cb_ix in cb_ixs {
                ixs.insert(0, cb_ix);
            }
            ixs
        }
    };
    let RecentBlockhash { hash, .. } = rpc.get_confirmed_blockhash().await.unwrap();
    rpc.handle_tx(
        &VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(&payer_pk, &ixs, luts, hash).unwrap()),
            &SortedSigners(signers),
        )
        .unwrap(),
        send_mode,
        HandleTxArgs::cli_default(),
    )
    .await;
}
