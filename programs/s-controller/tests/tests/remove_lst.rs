use s_controller_interface::{remove_lst_ix, LstState, RemoveLstIxArgs};
use s_controller_lib::{
    create_pool_reserves_address, create_protocol_fee_accumulator_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, RemoveLstFreeArgs,
};
use sanctum_solana_test_utils::{
    test_fixtures_dir,
    token::{tokenkeg::TokenkegProgramTest, MockMintArgs},
};
use solana_program::{clock::Clock, hash::Hash, pubkey::Pubkey};
use solana_program_test::{BanksClient, ProgramTestContext};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use test_utils::JITO_STAKE_POOL_LAST_UPDATE_EPOCH;

use crate::common::*;

#[tokio::test]
async fn basic_two_clear_from_front() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
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

    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
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

#[tokio::test]
async fn basic_three_clear_1_0_2() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let random_lst_states = [0; 3].map(|_| MockLstStateArgs {
        mint: Pubkey::new_unique(),
        token_program: spl_token::ID,
        sol_value_calculator: Pubkey::default(),
        sol_value: 0,
        reserves_amt: 0,
        protocol_fee_accumulator_amt: 0,
    });

    let mut program_test =
        naked_pool_state_program_test(DEFAULT_POOL_STATE).add_mock_lst_states(&random_lst_states);
    for s in random_lst_states {
        program_test = program_test.add_tokenkeg_mint_from_args(
            s.mint,
            MockMintArgs {
                mint_authority: None,
                freeze_authority: None,
                supply: 0,
                decimals: 9,
            },
        );
    }

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    exec_verify_remove(&mut banks_client, 1, &payer, last_blockhash, &mock_auth_kp).await;
    exec_verify_remove(&mut banks_client, 0, &payer, last_blockhash, &mock_auth_kp).await;
    // only initial lst_state_list[2] remains
    exec_verify_remove(&mut banks_client, 0, &payer, last_blockhash, &mock_auth_kp).await;
}

async fn exec_verify_remove(
    banks_client: &mut BanksClient,
    lst_index: usize,
    payer: &Keypair,
    last_blockhash: Hash,
    mock_auth_kp: &Keypair,
) {
    let lst_state_list_acc = banks_client.get_lst_state_list_acc().await;
    let og_lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
    let og_len = og_lst_state_list.len();
    let lst_state = og_lst_state_list[lst_index];
    let expected_lst_state_list_post: Vec<LstState> = og_lst_state_list
        .iter()
        .filter_map(|s| if *s == lst_state { None } else { Some(*s) })
        .collect();

    let mint_acc = banks_client
        .get_account(lst_state.mint)
        .await
        .unwrap()
        .unwrap();
    let lst_token_program = mint_acc.owner;

    let keys = RemoveLstFreeArgs {
        lst_index,
        refund_rent_to: payer.pubkey(),
        pool_state: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: banks_client.get_pool_state_acc().await,
        },
        lst_state_list: KeyedAccount {
            pubkey: LST_STATE_LIST_ID,
            account: lst_state_list_acc,
        },
        lst_mint: KeyedAccount {
            pubkey: lst_state.mint,
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

    let expected_new_len = og_len - 1;
    if expected_new_len == 0 {
        assert!(banks_client
            .get_account(LST_STATE_LIST_ID)
            .await
            .unwrap()
            .is_none())
    } else {
        let lst_state_list_acc = banks_client.get_lst_state_list_acc().await;
        let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
        assert_eq!(lst_state_list, expected_lst_state_list_post);
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
