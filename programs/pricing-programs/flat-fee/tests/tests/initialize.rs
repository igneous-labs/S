use flat_fee_interface::initialize_ix;
use flat_fee_lib::{account_resolvers::InitializeFreeArgs, utils::try_program_state};
use flat_fee_test_utils::{FlatFeePricingProgramTestBanksClient, DEFAULT_PROGRAM_STATE};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{signer::Signer, transaction::Transaction};

#[tokio::test]
async fn initialize_basic() {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "flat_fee",
        flat_fee_lib::program::ID,
        processor!(flat_fee::entrypoint::process_instruction),
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = initialize_ix(
        InitializeFreeArgs {
            payer: payer.pubkey(),
        }
        .resolve(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let state_acc = banks_client.get_flat_fee_program_state().await;
    let state = try_program_state(&state_acc.data).unwrap();
    assert_eq!(*state, DEFAULT_PROGRAM_STATE);
}
