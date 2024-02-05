use s_controller_interface::{disable_pool_ix, SControllerError};
use s_controller_lib::{try_pool_state, DisablePoolFreeArgs, U8Bool};
use s_controller_test_utils::{
    DisablePoolAuthorityListProgramTest, PoolStateBanksClient, PoolStateProgramTest,
    DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::assert_custom_err;
use solana_program_test::ProgramTest;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::*;

#[tokio::test]
async fn basic_disable_pool() {
    let disable_pool_authority_kp = Keypair::new();

    let program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE)
        .add_disable_pool_authority_list(&[disable_pool_authority_kp.pubkey()]);

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
    let disable_pool_authority_kp = Keypair::new();

    let program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE)
        .add_disable_pool_authority_list(&[disable_pool_authority_kp.pubkey()]);

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
