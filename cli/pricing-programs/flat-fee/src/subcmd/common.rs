use data_encoding::BASE64;
use flat_fee_interface::ProgramState;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_response::RpcSimulateTransactionResult,
};
use solana_sdk::{
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    native_token::lamports_to_sol,
    pubkey::Pubkey,
    signer::Signer,
    transaction::VersionedTransaction,
};
use solana_transaction_status::{UiReturnDataEncoding, UiTransactionReturnData};
use std::convert::Infallible;

pub fn verify_manager(state: &ProgramState, curr_manager: Pubkey) -> Result<(), Infallible> {
    if state.manager != curr_manager {
        eprintln!(
            "Wrong manager. Expected: {}. Got: {}",
            state.manager, curr_manager
        );
        std::process::exit(-1);
    }
    Ok(())
}

pub async fn handle_pricing_ix(rpc: &RpcClient, ix: Instruction, payer: &dyn Signer) {
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
    let data: &[u8; 8] = data.as_slice().try_into().unwrap();
    let sol_value = u64::from_le_bytes(*data);
    println!("{}", lamports_to_sol(sol_value));
}
