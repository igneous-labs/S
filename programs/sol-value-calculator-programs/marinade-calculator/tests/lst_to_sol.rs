use generic_pool_calculator_interface::{LstToSolIxArgs, LstToSolKeys};
use marinade_calculator_lib::{
    marinade_lst_to_sol_ix, MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use solana_program_test::BanksTransactionResultWithMetadata;
use solana_sdk::{
    signer::Signer, transaction::Transaction, transaction_context::TransactionReturnData,
};
use test_utils::zero_padded_return_data;

use crate::common::marinade_normal_program_test;

mod common;

#[tokio::test]
async fn basic() {
    const LST_AMOUNT: u64 = 1_000_000_000;
    const EXPECTED_LAMPORTS_AMOUNT: u64 = 1_151_526_823;

    let program_test = marinade_normal_program_test();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let accounts: LstToSolKeys = MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
        .resolve::<MarinadeSolValCalc>()
        .into();

    let ix = marinade_lst_to_sol_ix(accounts, LstToSolIxArgs { amount: LST_AMOUNT }).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let BanksTransactionResultWithMetadata { result, metadata } = banks_client
        .process_transaction_with_metadata(tx)
        .await
        .unwrap();

    assert!(result.is_ok());
    let TransactionReturnData { program_id, data } = metadata.unwrap().return_data.unwrap();
    assert_eq!(program_id, marinade_calculator_lib::program::ID);
    let lamports = u64::from_le_bytes(zero_padded_return_data(&data));
    assert_eq!(lamports, EXPECTED_LAMPORTS_AMOUNT);
}
