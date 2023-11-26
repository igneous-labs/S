use generic_pool_calculator_interface::{LstToSolIxArgs, LstToSolKeys};
use solana_program_test::BanksTransactionResultWithMetadata;
use solana_sdk::{
    signer::Signer, transaction::Transaction, transaction_context::TransactionReturnData,
};
use spl_calculator_lib::{
    account_resolvers::SplLstSolCommonRootAccounts, spl_lst_to_sol_ix, SplSolValCalc,
};
use test_utils::zero_padded_return_data;

use crate::common::{jito_normal_program_test, JitoNormalProgramTest};

mod common;

#[tokio::test]
async fn jito_basic() {
    const LST_AMOUNT: u64 = 1_000_000_000;
    const EXPECTED_LAMPORTS_AMOUNT: u64 = 1_071_477_406;

    let JitoNormalProgramTest {
        program_test,
        jito_stake_pool,
        spl_stake_pool_prog,
    } = jito_normal_program_test();

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let root_accounts = SplLstSolCommonRootAccounts {
        spl_stake_pool: jito_stake_pool,
        spl_stake_pool_prog,
    };
    let (intermediate, _stake_pool) = root_accounts.resolve().unwrap();
    let accounts: LstToSolKeys = intermediate.resolve::<SplSolValCalc>().unwrap().into();

    let ix = spl_lst_to_sol_ix(accounts, LstToSolIxArgs { amount: LST_AMOUNT }).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);

    let BanksTransactionResultWithMetadata { result, metadata } = banks_client
        .process_transaction_with_metadata(tx)
        .await
        .unwrap();

    assert!(result.is_ok());
    let TransactionReturnData { program_id, data } = metadata.unwrap().return_data.unwrap();
    assert_eq!(program_id, spl_calculator_lib::program::ID);
    let lamports = u64::from_le_bytes(zero_padded_return_data(&data));
    assert_eq!(lamports, EXPECTED_LAMPORTS_AMOUNT);
}
