use s_controller_interface::{add_disable_pool_authority_ix, SControllerError};
use s_controller_lib::{
    program::POOL_STATE_ID, try_disable_pool_authority_list, try_match_element_in_list,
    AddDisablePoolAuthorityFreeArgs,
};
use sanctum_solana_test_utils::{assert_custom_err, test_fixtures_dir, IntoAccount};
use solana_program_test::{processor, ProgramTest};
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

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

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
async fn reject_duplicates() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let program_test = naked_pool_state_program_test(DEFAULT_POOL_STATE);
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = MockPoolState(DEFAULT_POOL_STATE).into_account();
    let new_authority_keypair = Keypair::new();
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

    assert_eq!(disable_pool_authority_list.len(), 1);
    assert_eq!(
        disable_pool_authority_list[0],
        new_authority_keypair.pubkey()
    );

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

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_custom_err(err, SControllerError::DuplicateDisablePoolAuthority);
}
