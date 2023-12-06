mod common;

use common::*;
use s_controller_interface::{enable_pool_ix, EnablePoolIxArgs, PoolState};
use s_controller_lib::{program::POOL_STATE_ID, try_pool_state, EnablePoolFreeArgs, U8Bool};
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};
use test_utils::test_fixtures_dir;

#[tokio::test]
async fn basic_enable_pool() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );

    // TODO: confirm this syntax
    let pool_state_account = pool_state_to_account(PoolState {
        is_disabled: 1,
        ..DEFAULT_POOL_STATE
    });
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        pool_state_account.clone(),
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // Enable pool
    {
        let keys = EnablePoolFreeArgs {
            pool_state_acc: KeyedReadonlyAccount {
                key: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
        }
        .resolve()
        .unwrap();
        let ix = enable_pool_ix(keys, EnablePoolIxArgs {}).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

        assert!(U8Bool(pool_state.is_disabled).is_false());
    }
}
