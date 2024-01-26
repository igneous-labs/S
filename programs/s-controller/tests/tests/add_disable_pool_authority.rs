use s_controller_interface::{add_disable_pool_authority_ix, SControllerError};
use s_controller_lib::{
    program::POOL_STATE_ID, try_disable_pool_authority_list, try_match_element_in_list,
    AddDisablePoolAuthorityFreeArgs,
};
use sanctum_solana_test_utils::{assert_custom_err, test_fixtures_dir, IntoAccount};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};

use crate::common::*;

#[tokio::test]
async fn basic_add_two() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let program_test = naked_pool_state_program_test(DEFAULT_POOL_STATE);
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = MockPoolState(DEFAULT_POOL_STATE).into_account();

    // Add an authority
    {
        let new_authority_keypair = Keypair::new();
        let expected_index = 0;
        let keys = AddDisablePoolAuthorityFreeArgs {
            payer: payer.pubkey(),
            new_authority: new_authority_keypair.pubkey(),
            pool_state_acc: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
        }
        .resolve()
        .unwrap();

        let ix = add_disable_pool_authority_ix(keys).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let disable_pool_authority_list_acc = banks_client.get_disable_pool_list_acc().await;
        let disable_pool_authority_list =
            try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();
        assert_eq!(disable_pool_authority_list.len(), expected_index + 1);
        // verify index and element in list
        try_match_element_in_list(
            new_authority_keypair.pubkey(),
            disable_pool_authority_list,
            expected_index,
        )
        .unwrap();
    }

    // Add another authority
    {
        let new_authority_keypair = Keypair::new();
        let expected_index = 1;
        let keys = AddDisablePoolAuthorityFreeArgs {
            payer: payer.pubkey(),
            new_authority: new_authority_keypair.pubkey(),
            pool_state_acc: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
        }
        .resolve()
        .unwrap();

        let ix = add_disable_pool_authority_ix(keys).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let disable_pool_authority_list_acc = banks_client.get_disable_pool_list_acc().await;
        let disable_pool_authority_list =
            try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();
        assert_eq!(disable_pool_authority_list.len(), expected_index + 1);
        // verify index and element in list
        try_match_element_in_list(
            new_authority_keypair.pubkey(),
            disable_pool_authority_list,
            expected_index,
        )
        .unwrap();
    }
}

#[tokio::test]
async fn fail_add_duplicates() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let existing_authority_keypair = Keypair::new();
    let program_test = naked_pool_state_program_test(DEFAULT_POOL_STATE)
        .add_disable_pool_authority_list(&[existing_authority_keypair.pubkey()]);
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = MockPoolState(DEFAULT_POOL_STATE).into_account();
    let keys = AddDisablePoolAuthorityFreeArgs {
        payer: payer.pubkey(),
        new_authority: existing_authority_keypair.pubkey(),
        pool_state_acc: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: pool_state_account.clone(),
        },
    }
    .resolve()
    .unwrap();

    let ix = add_disable_pool_authority_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_custom_err(err, SControllerError::DuplicateDisablePoolAuthority);

    let disable_pool_authority_list_acc = banks_client.get_disable_pool_list_acc().await;
    let disable_pool_authority_list =
        try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();

    assert_eq!(disable_pool_authority_list.len(), 1);
    assert_eq!(
        disable_pool_authority_list[0],
        existing_authority_keypair.pubkey()
    );
}
