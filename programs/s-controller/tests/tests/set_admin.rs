use s_controller_interface::set_admin_ix;
use s_controller_lib::{program::POOL_STATE_ID, SetAdminFreeArgs};

use s_controller_test_utils::{
    assert_admin, PoolStateBanksClient, PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::test_fixtures_dir;
use solana_program_test::*;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};

use crate::common::*;

#[tokio::test]
async fn basic_set_admin() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let new_admin_kp = Keypair::new();
    let another_new_admin_kp = Keypair::new();

    let program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // Change admin
    let ix = set_admin_ix(
        SetAdminFreeArgs {
            new_admin: new_admin_kp.pubkey(),
            pool_state: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: banks_client.get_pool_state_acc().await,
            },
        }
        .resolve()
        .unwrap(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    assert_admin(&mut banks_client, new_admin_kp.pubkey()).await;

    // Change admin again
    let ix2 = set_admin_ix(
        SetAdminFreeArgs {
            new_admin: another_new_admin_kp.pubkey(),
            pool_state: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: banks_client.get_pool_state_acc().await,
            },
        }
        .resolve()
        .unwrap(),
    )
    .unwrap();

    let mut tx2 = Transaction::new_with_payer(&[ix2], Some(&payer.pubkey()));
    tx2.sign(&[&payer, &new_admin_kp], last_blockhash);

    banks_client.process_transaction(tx2).await.unwrap();

    assert_admin(&mut banks_client, another_new_admin_kp.pubkey()).await;
}
