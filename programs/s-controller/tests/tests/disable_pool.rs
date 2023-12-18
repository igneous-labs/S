use s_controller_interface::{disable_pool_ix, SControllerError};
use s_controller_lib::{try_pool_state, DisablePoolFreeArgs, U8Bool};
use sanctum_solana_test_utils::{assert_custom_err, IntoAccount};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::*;

#[tokio::test]
async fn basic_disable_pool() {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );

    let pool_state_account = MockPoolState(DEFAULT_POOL_STATE).into_account();
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        pool_state_account.clone(),
    );

    let disable_pool_authority_kp = Keypair::new();
    program_test =
        program_test.add_disable_pool_authority_list(&[disable_pool_authority_kp.pubkey()]);

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // Disable pool by disable pool authority
    {
        let keys = DisablePoolFreeArgs {
            signer: disable_pool_authority_kp.pubkey(),
        }
        .resolve();
        let ix = disable_pool_ix(keys).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &disable_pool_authority_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let pool_state_acc = banks_client.get_pool_state_acc().await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

        assert!(U8Bool(pool_state.is_disabled).is_true());
    }
}

#[tokio::test]
async fn reject_disable_pool() {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );

    let pool_state_account = MockPoolState(DEFAULT_POOL_STATE).into_account();
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        pool_state_account.clone(),
    );

    let disable_pool_authority_kp = Keypair::new();
    program_test =
        program_test.add_disable_pool_authority_list(&[disable_pool_authority_kp.pubkey()]);

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // Disable pool by disable pool authority
    {
        // A keypair not authorized to disable pool
        let rando_kp = Keypair::new();
        let keys = DisablePoolFreeArgs {
            signer: rando_kp.pubkey(),
        }
        .resolve();
        let ix = disable_pool_ix(keys).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &rando_kp], last_blockhash);

        let err = banks_client.process_transaction(tx).await.unwrap_err();

        assert_custom_err(err, SControllerError::InvalidDisablePoolAuthority);

        let pool_state_acc = banks_client.get_pool_state_acc().await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

        assert!(U8Bool(pool_state.is_disabled).is_false());
    }
}
