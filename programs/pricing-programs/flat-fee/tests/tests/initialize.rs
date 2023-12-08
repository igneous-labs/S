use solana_program_test::{processor, ProgramTest};

use crate::common::*;

#[tokio::test]
async fn basic() {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "flat_fee",
        flat_fee_lib::program::ID,
        processor!(flat_fee::entrypoint::process_instruction),
    );

    let program_state_account = program_state_to_account(DEFAULT_PROGRAM_STATE);
    program_test.add_account(
        flat_fee_lib::program::STATE_ID,
        program_state_account.clone(),
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // TODO: write test
    {}
}
