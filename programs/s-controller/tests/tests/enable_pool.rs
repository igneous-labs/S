use s_controller_interface::{enable_pool_ix, PoolState};
use s_controller_lib::{program::POOL_STATE_ID, try_pool_state, EnablePoolFreeArgs, U8Bool};
use s_controller_test_utils::{
    MockPoolState, PoolStateBanksClient, PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::{test_fixtures_dir, IntoAccount};
use solana_program_test::ProgramTest;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};

use crate::common::*;

#[tokio::test]
async fn basic_enable_pool() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let pool_state = PoolState {
        is_disabled: 1,
        ..DEFAULT_POOL_STATE
    };
    let program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(pool_state);

    let pool_state_account = MockPoolState(pool_state).into_account();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // Enable pool
    {
        let keys = EnablePoolFreeArgs {
            pool_state_acc: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_account,
            },
        }
        .resolve()
        .unwrap();
        let ix = enable_pool_ix(keys).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let pool_state_acc = banks_client.get_pool_state_acc().await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

        assert!(U8Bool(pool_state.is_disabled).is_false());
    }
}
