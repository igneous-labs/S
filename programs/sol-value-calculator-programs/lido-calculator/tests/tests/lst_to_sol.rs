use generic_pool_calculator_interface::{LstToSolIxArgs, LstToSolKeys};
use lido_calculator_lib::{
    lido_lst_to_sol_ix, LidoSolValCalc, LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use solana_program::clock::Clock;
use solana_program_test::{BanksTransactionResultWithMetadata, ProgramTestContext};
use solana_sdk::{
    signer::Signer, transaction::Transaction, transaction_context::TransactionReturnData,
};
use test_utils::{zero_padded_return_data, LIDO_STATE_LAST_UPDATE_EPOCH};

use crate::common::lido_normal_program_test;

#[tokio::test]
async fn basic() {
    const LST_AMOUNT: u64 = 1_000_000_000;
    const EXPECTED_LAMPORTS_AMOUNT: u64 = 1_147_696_330;

    let program_test = lido_normal_program_test();

    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: LIDO_STATE_LAST_UPDATE_EPOCH,
        ..Default::default()
    });

    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    let accounts: LstToSolKeys = LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS
        .resolve::<LidoSolValCalc>()
        .into();

    let ix = lido_lst_to_sol_ix(accounts, LstToSolIxArgs { amount: LST_AMOUNT }).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let BanksTransactionResultWithMetadata { result, metadata } = banks_client
        .process_transaction_with_metadata(tx)
        .await
        .unwrap();

    assert!(result.is_ok());
    let TransactionReturnData { program_id, data } = metadata.unwrap().return_data.unwrap();
    assert_eq!(program_id, lido_calculator_lib::program::ID);
    let lamports = u64::from_le_bytes(zero_padded_return_data(&data));
    assert_eq!(lamports, EXPECTED_LAMPORTS_AMOUNT);
}
