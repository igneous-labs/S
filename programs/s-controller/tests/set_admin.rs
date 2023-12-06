use s_controller_interface::{set_admin_ix, PoolState, SetAdminIxArgs};
use s_controller_lib::{
    initial_authority, program::POOL_STATE_ID, try_pool_state, SetAdminFreeArgs,
};
use solana_program::system_program;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use test_utils::test_fixtures_dir;

mod common;

use crate::common::*;

#[tokio::test]
async fn basic_set_admin() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let new_admin_kp = Keypair::new();

    let mut program_test = ProgramTest::default();

    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );

    program_test.add_account(POOL_STATE_ID, pool_state_to_account(DEFAULT_POOL_STATE));

    program_test.add_account(
        initial_authority::ID,
        Account {
            lamports: 5,
            data: vec![0; 1024], // Adjust size as needed
            owner: system_program::ID,
            ..Account::default()
        },
    );

    program_test.add_account(
        new_admin_kp.pubkey(),
        Account {
            lamports: 5,
            data: vec![0; 1024], // Adjust size as needed
            owner: system_program::ID,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_admin_ix(
        SetAdminFreeArgs {
            new_admin: new_admin_kp.pubkey(),
        }
        .resolve(),
        SetAdminIxArgs {},
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

    assert_eq!(
        *pool_state,
        PoolState {
            admin: new_admin_kp.pubkey(),
            ..*pool_state
        }
    );
}
