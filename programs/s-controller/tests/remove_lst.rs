use s_controller_interface::{remove_lst_ix, LstState, RemoveLstIxArgs};
use s_controller_lib::{
    create_pool_reserves_address, create_protocol_fee_accumulator_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, RemoveLstFreeArgs,
};
use solana_program::{clock::Clock, hash::Hash, pubkey::Pubkey};
use solana_program_test::{BanksClient, ProgramTestContext};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use test_utils::{test_fixtures_dir, JITO_STAKE_POOL_LAST_UPDATE_EPOCH};

mod common;

use common::*;

#[tokio::test]
async fn basic_two_clear_from_front() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let program_test = jito_marinade_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
    });
    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });
    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    exec_verify_remove(&mut banks_client, 0, &payer, last_blockhash, &mock_auth_kp).await;
    exec_verify_remove(&mut banks_client, 0, &payer, last_blockhash, &mock_auth_kp).await;
}

#[tokio::test]
async fn basic_two_clear_from_back() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let program_test = jito_marinade_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
    });
    let ctx = program_test.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });
    let ProgramTestContext {
        mut banks_client,
        last_blockhash,
        payer,
        ..
    } = ctx;

    exec_verify_remove(&mut banks_client, 1, &payer, last_blockhash, &mock_auth_kp).await;
    exec_verify_remove(&mut banks_client, 0, &payer, last_blockhash, &mock_auth_kp).await;
}

async fn exec_verify_remove(
    banks_client: &mut BanksClient,
    lst_index: usize,
    payer: &Keypair,
    last_blockhash: Hash,
    mock_auth_kp: &Keypair,
) {
    let lst_state_list_acc = banks_client_get_lst_state_list_acc(banks_client).await;
    let (lst_state, original_len) = {
        let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
        (lst_state_list[lst_index], lst_state_list.len())
    };
    let mint_acc = banks_client
        .get_account(lst_state.mint)
        .await
        .unwrap()
        .unwrap();
    let lst_token_program = mint_acc.owner;

    let keys = RemoveLstFreeArgs {
        lst_index,
        refund_rent_to: payer.pubkey(),
        pool_state: KeyedReadonlyAccount {
            key: POOL_STATE_ID,
            account: banks_client_get_pool_state_acc(banks_client).await,
        },
        lst_state_list: KeyedReadonlyAccount {
            key: LST_STATE_LIST_ID,
            account: lst_state_list_acc,
        },
        lst_mint: KeyedReadonlyAccount {
            key: lst_state.mint,
            account: mint_acc,
        },
    }
    .resolve()
    .unwrap();
    let ix = remove_lst_ix(
        keys,
        RemoveLstIxArgs {
            lst_index: lst_index.try_into().unwrap(),
        },
    )
    .unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[payer, mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let expected_new_len = original_len - 1;
    if expected_new_len == 0 {
        assert!(banks_client
            .get_account(LST_STATE_LIST_ID)
            .await
            .unwrap()
            .is_none())
    } else {
        let lst_state_list_acc = banks_client_get_lst_state_list_acc(banks_client).await;
        let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
        assert_eq!(lst_state_list.len(), expected_new_len);
        assert!(try_find_lst_mint_on_list(lst_state.mint, lst_state_list).is_err());
    }
    verify_lst_token_accounts_deleted(banks_client, lst_state, lst_token_program).await;
}

async fn verify_lst_token_accounts_deleted(
    banks_client: &mut BanksClient,
    lst_state: LstState,
    lst_token_program: Pubkey,
) {
    let pool_reserves_addr = create_pool_reserves_address(&lst_state, lst_token_program).unwrap();
    let protocol_fee_accumulator_addr =
        create_protocol_fee_accumulator_address(&lst_state, lst_token_program).unwrap();
    for should_be_deleted in [pool_reserves_addr, protocol_fee_accumulator_addr] {
        assert!(banks_client
            .get_account(should_be_deleted)
            .await
            .unwrap()
            .is_none())
    }
}
