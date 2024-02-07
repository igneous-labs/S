use s_controller_interface::{set_pricing_program_ix, SControllerError};
use s_controller_lib::{
    program::POOL_STATE_ID, try_pool_state, SetPricingProgramFreeArgs, DEFAULT_PRICING_PROGRAM,
};

use s_controller_test_utils::{
    assert_pricing_prog_set, PoolStateBanksClient, PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::{assert_custom_err, assert_program_error, test_fixtures_dir};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};

use crate::common::SControllerProgramTest;

fn no_fee_program_test() -> (ProgramTest, Keypair) {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let mut program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);
    program_test.add_program(
        "no_fee_pricing_program",
        no_fee_pricing_program::ID,
        processor!(no_fee_pricing_program::process_instruction),
    );
    assert_ne!(no_fee_pricing_program::ID, DEFAULT_PRICING_PROGRAM);
    (program_test, mock_auth_kp)
}

#[tokio::test]
async fn basic_success() {
    let (program_test, mock_auth_kp) = no_fee_program_test();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let keys = SetPricingProgramFreeArgs {
        new_pricing_program: no_fee_pricing_program::ID,
        pool_state_acc: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: pool_state_account,
        },
    }
    .resolve()
    .unwrap();
    let ix = set_pricing_program_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    assert_pricing_prog_set(&mut banks_client, no_fee_pricing_program::ID).await;
}

#[tokio::test]
async fn fail_unauthorized() {
    let (program_test, _mock_auth_kp) = no_fee_program_test();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // A non-admin keypair
    let rando_kp = Keypair::new();

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let mut keys = SetPricingProgramFreeArgs {
        new_pricing_program: no_fee_pricing_program::ID,
        pool_state_acc: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: pool_state_account,
        },
    }
    .resolve()
    .unwrap();

    keys.admin = rando_kp.pubkey();

    let ix = set_pricing_program_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &rando_kp], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    let pool_state_acc = banks_client.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

    assert_program_error(err, ProgramError::InvalidArgument);
    assert_ne!(pool_state.pricing_program, no_fee_pricing_program::ID);
}

#[tokio::test]
async fn fail_invalid_program() {
    let (program_test, mock_auth_kp) = no_fee_program_test();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let uninitialized_pricing_program = Pubkey::new_unique();
    let keys = SetPricingProgramFreeArgs {
        new_pricing_program: uninitialized_pricing_program,
        pool_state_acc: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: pool_state_account,
        },
    }
    .resolve()
    .unwrap();
    let ix = set_pricing_program_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_custom_err(err, SControllerError::FaultyPricingProgram);

    let pool_state_acc = banks_client.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

    assert_ne!(pool_state.pricing_program, uninitialized_pricing_program);
}
