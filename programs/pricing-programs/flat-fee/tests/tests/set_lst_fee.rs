use flat_fee_interface::{set_lst_fee_ix, SetLstFeeIxArgs};
use flat_fee_lib::account_resolvers::SetLstFeeFreeArgs;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};
use test_utils::test_fixtures_dir;

use crate::common::*;

#[tokio::test]
async fn basic() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("flat-fee-test-initial-manager-key.json"))
            .unwrap();
    // TODO: mock_lst_mint

    let mut program_test = ProgramTest::default();

    program_test.add_program(
        "flat_fee",
        flat_fee_lib::program::ID,
        processor!(flat_fee::entrypoint::process_instruction),
    );
    // TODO: add FeeAccount for mock_lst_mint

    let program_state_acc = program_state_to_account(DEFAULT_PROGRAM_STATE);
    program_test.add_account(flat_fee_lib::program::STATE_ID, program_state_acc.clone());

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // set lst fee
    // {
    //     let ix = set_lst_fee_ix(
    //         SetLstFeeFreeArgs {
    //             fee_acc: todo!(),
    //             state_acc: todo!(),
    //         }
    //         .resolve()
    //         .unwrap(),
    //         SetLstFeeIxArgs {
    //             input_fee_bps: todo!(),
    //             output_fee_bps: todo!(),
    //         },
    //     )
    //     .unwrap();

    //     let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    //     tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    //     banks_client.process_transaction(tx).await.unwrap();
    // }
}
