mod common;

use common::*;
use s_controller_interface::{
    set_rebalance_authority_ix, SControllerError, SetRebalanceAuthorityIxArgs,
};
use s_controller_lib::{
    program::POOL_STATE_ID, try_pool_state, KnownAuthoritySetRebalanceAuthorityFreeArgs,
    SetRebalanceAuthorityFreeArgs,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    account::Account,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use test_utils::{assert_is_custom_err, test_fixtures_dir};

#[tokio::test]
async fn admin_set() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let new_rebalance_authority = Pubkey::new_unique();

    let pool_state_acc = pool_state_to_account(DEFAULT_POOL_STATE);
    let program_test = naked_pool_state_program_test(pool_state_acc.clone());
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_rebalance_authority_ix(
        KnownAuthoritySetRebalanceAuthorityFreeArgs {
            new_rebalance_authority,
            pool_state: pool_state_acc,
        }
        .resolve_pool_admin()
        .unwrap(),
        SetRebalanceAuthorityIxArgs {},
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    verify_new_rebalance_authority(&mut banks_client, new_rebalance_authority).await;
}

#[tokio::test]
async fn rebalance_authority_set() {
    let current_rebalance_authority = Keypair::new();
    let new_rebalance_authority = Pubkey::new_unique();

    let mut pool_state = DEFAULT_POOL_STATE;
    pool_state.rebalance_authority = current_rebalance_authority.pubkey();

    let pool_state_acc = pool_state_to_account(pool_state);
    let program_test = naked_pool_state_program_test(pool_state_acc.clone());
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_rebalance_authority_ix(
        KnownAuthoritySetRebalanceAuthorityFreeArgs {
            new_rebalance_authority,
            pool_state: pool_state_acc,
        }
        .resolve_current_rebalance_authority()
        .unwrap(),
        SetRebalanceAuthorityIxArgs {},
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &current_rebalance_authority], last_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    verify_new_rebalance_authority(&mut banks_client, new_rebalance_authority).await;
}

#[tokio::test]
async fn unauthorized_signer() {
    let new_rebalance_authority = Pubkey::new_unique();

    let pool_state_acc = pool_state_to_account(DEFAULT_POOL_STATE);
    let program_test = naked_pool_state_program_test(pool_state_acc.clone());
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_rebalance_authority_ix(
        SetRebalanceAuthorityFreeArgs {
            new_rebalance_authority,
            signer: payer.pubkey(), // payer is unauthorized
        }
        .resolve(),
        SetRebalanceAuthorityIxArgs {},
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);
    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_is_custom_err(
        err,
        SControllerError::UnauthorizedSetRebalanceAuthoritySigner,
    );
}

fn naked_pool_state_program_test(pool_state_account: Account) -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );
    program_test.add_account(POOL_STATE_ID, pool_state_account);
    program_test
}

async fn verify_new_rebalance_authority(
    banks_client: &mut BanksClient,
    expected_new_rebalance_authority: Pubkey,
) {
    let pool_state_account = banks_client_get_pool_state_acc(banks_client).await;
    let pool_state = try_pool_state(&pool_state_account.data).unwrap();
    assert_eq!(
        pool_state.rebalance_authority,
        expected_new_rebalance_authority
    );
}
