use generic_pool_calculator_interface::{LstToSolIxArgs, LstToSolKeys};
use solana_program::clock::Clock;
use solana_program_test::{BanksTransactionResultWithMetadata, ProgramTestContext};
use solana_sdk::{
    signer::Signer, transaction::Transaction, transaction_context::TransactionReturnData,
};
use spl_calculator_lib::{spl_lst_to_sol_ix, SplLstSolCommonFreeArgs, SplSolValCalc};
use test_utils::{zero_padded_return_data, JITO_STAKE_POOL_LAST_UPDATE_EPOCH};

use crate::common::{jito_normal_program_test, JitoNormalProgramTest};

mod common;

#[tokio::test]
async fn jito_basic() {
    const LST_AMOUNT: u64 = 1_000_000_000;
    const EXPECTED_LAMPORTS_AMOUNT: u64 = 1_072_326_756;

    let JitoNormalProgramTest {
        program_test,
        jito_stake_pool,
        spl_stake_pool_prog,
    } = jito_normal_program_test();

    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });

    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    let free_args = SplLstSolCommonFreeArgs {
        spl_stake_pool: jito_stake_pool,
        spl_stake_pool_prog,
    };
    let (intermediate, _stake_pool) = free_args.resolve().unwrap();
    let accounts: LstToSolKeys = intermediate.resolve::<SplSolValCalc>().unwrap().into();

    let ix = spl_lst_to_sol_ix(accounts, LstToSolIxArgs { amount: LST_AMOUNT }).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

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
