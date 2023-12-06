mod common;

use common::*;
use s_controller_interface::{
    remove_disable_pool_authority_ix, RemoveDisablePoolAuthorityIxArgs, SControllerError,
};
use s_controller_lib::{
    program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID},
    try_disable_pool_authority_list, try_match_element_in_list, RemoveDisablePoolAuthorityFreeArgs,
};
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use test_utils::test_fixtures_dir;

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

    let pool_state_account = pool_state_to_account(DEFAULT_POOL_STATE);
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        pool_state_account.clone(),
    );

    // NOTE: assumes keypairs are unique
    let disable_pool_authority_kps = [Keypair::new(), Keypair::new(), Keypair::new()];
    let disable_pool_authority_pks: Vec<_> = disable_pool_authority_kps
        .iter()
        .map(|k| k.pubkey())
        .collect();
    program_test =
        program_test_add_disable_pool_authority_list(program_test, &disable_pool_authority_pks);

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let disable_pool_authority_list_acc =
        banks_client_get_disable_pool_list_acc(&mut banks_client).await;
    let disable_pool_authority_list =
        try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();

    // Remove an authority by the admin
    {
        let before_len = disable_pool_authority_list.len();
        let target_index = 0;
        let target_authority = disable_pool_authority_pks[target_index];
        let keys = RemoveDisablePoolAuthorityFreeArgs {
            index: target_index,
            refund_rent_to: payer.pubkey(),
            signer: mock_auth_kp.pubkey(),
            authority: target_authority,
            pool_state_acc: KeyedReadonlyAccount {
                key: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
            disable_pool_authority_list: KeyedReadonlyAccount {
                key: DISABLE_POOL_AUTHORITY_LIST_ID,
                account: disable_pool_authority_list_acc.clone(),
            },
        }
        .resolve()
        .unwrap();

        let ix = remove_disable_pool_authority_ix(
            keys,
            RemoveDisablePoolAuthorityIxArgs {
                index: target_index as u32, // TODO: use index_to_u32 after merge
            },
        )
        .unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let disable_pool_authority_list_acc =
            banks_client_get_disable_pool_list_acc(&mut banks_client).await;
        let disable_pool_authority_list =
            try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();
        assert_eq!(disable_pool_authority_list.len(), before_len - 1);
        let result =
            try_match_element_in_list(target_authority, disable_pool_authority_list, target_index);
        assert_eq!(
            result,
            Err(SControllerError::InvalidDisablePoolAuthorityIndex)
        );
    }

    // Remove an authority by the authority
    {
        let before_len = disable_pool_authority_list.len();
        let target_index = 1;
        let target_authority_kp = &disable_pool_authority_kps[target_index];
        let keys = RemoveDisablePoolAuthorityFreeArgs {
            index: target_index,
            refund_rent_to: payer.pubkey(),
            signer: target_authority_kp.pubkey(),
            authority: target_authority_kp.pubkey(),
            pool_state_acc: KeyedReadonlyAccount {
                key: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
            disable_pool_authority_list: KeyedReadonlyAccount {
                key: DISABLE_POOL_AUTHORITY_LIST_ID,
                account: disable_pool_authority_list_acc.clone(),
            },
        }
        .resolve()
        .unwrap();

        let ix = remove_disable_pool_authority_ix(
            keys,
            RemoveDisablePoolAuthorityIxArgs {
                index: target_index as u32, // TODO: use index_to_u32 after merge
            },
        )
        .unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &target_authority_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let disable_pool_authority_list_acc =
            banks_client_get_disable_pool_list_acc(&mut banks_client).await;
        let disable_pool_authority_list =
            try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();
        assert_eq!(disable_pool_authority_list.len(), before_len - 1);
        let result = try_match_element_in_list(
            target_authority_kp.pubkey(),
            disable_pool_authority_list,
            target_index,
        );
        assert_eq!(
            result,
            Err(SControllerError::InvalidDisablePoolAuthorityIndex)
        );
    }
}
