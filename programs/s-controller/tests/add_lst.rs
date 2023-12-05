use marinade_keys::msol;
use s_controller_interface::{add_lst_ix, AddLstIxArgs, LstState};
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address,
    program::{POOL_STATE_ID, PROTOCOL_FEE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, AddLstFreeArgs, FindLstPdaAtaKeys,
};
use solana_program::{program_pack::Pack, pubkey::Pubkey};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};
use test_utils::{banks_client_get_account, jitosol, test_fixtures_dir, AddAccount};

mod common;

use common::*;

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

    program_test.add_program(
        "marinade_calculator",
        marinade_calculator_lib::program::ID,
        processor!(marinade_calculator::entrypoint::process_instruction),
    );
    program_test.add_program(
        "spl_calculator",
        spl_calculator_lib::program::ID,
        processor!(spl_calculator::entrypoint::process_instruction),
    );
    let mut program_test = program_test
        .add_test_fixtures_account("jitosol-mint.json")
        .add_test_fixtures_account("msol-mint.json");
    let pool_state_account = pool_state_to_account(DEFAULT_POOL_STATE);
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        pool_state_account.clone(),
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // Add jitoSOL

    let jitosol_mint_acc = banks_client_get_account(&mut banks_client, jitosol::ID).await;
    let (keys, _bumps) = AddLstFreeArgs {
        payer: payer.pubkey(),
        sol_value_calculator: spl_calculator_lib::program::ID,
        pool_state: KeyedReadonlyAccount {
            key: POOL_STATE_ID,
            account: pool_state_account.clone(),
        },
        lst_mint: KeyedReadonlyAccount {
            key: jitosol::ID,
            account: jitosol_mint_acc,
        },
    }
    .resolve()
    .unwrap();
    let ix = add_lst_ix(keys, AddLstIxArgs {}).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let lst_state_list_acc = banks_client_get_lst_state_list_acc(&mut banks_client).await;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
    assert_eq!(lst_state_list.len(), 1);
    verify_lst_added_success(
        &mut banks_client,
        lst_state_list,
        FindLstPdaAtaKeys {
            lst_mint: jitosol::ID,
            token_program: spl_token::ID,
        },
        spl_calculator_lib::program::ID,
        0,
    )
    .await;

    // Add mSOL

    let msol_mint_acc = banks_client_get_account(&mut banks_client, msol::ID).await;
    let (keys, _bumps) = AddLstFreeArgs {
        payer: payer.pubkey(),
        sol_value_calculator: marinade_calculator_lib::program::ID,
        pool_state: KeyedReadonlyAccount {
            key: POOL_STATE_ID,
            account: pool_state_account,
        },
        lst_mint: KeyedReadonlyAccount {
            key: msol::ID,
            account: msol_mint_acc,
        },
    }
    .resolve()
    .unwrap();
    let ix = add_lst_ix(keys, AddLstIxArgs {}).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let lst_state_list_acc = banks_client_get_lst_state_list_acc(&mut banks_client).await;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
    assert_eq!(lst_state_list.len(), 2);
    verify_lst_added_success(
        &mut banks_client,
        lst_state_list,
        FindLstPdaAtaKeys {
            lst_mint: msol::ID,
            token_program: spl_token::ID,
        },
        marinade_calculator_lib::program::ID,
        1,
    )
    .await;
}

async fn verify_lst_added_success(
    banks_client: &mut BanksClient,
    lst_state_list: &[LstState],
    find_lst_account_address_keys: FindLstPdaAtaKeys,
    expected_sol_value_calculator: Pubkey,
    expected_index: usize,
) {
    let (pool_reserves_addr, pool_reserves_bump) =
        find_pool_reserves_address(find_lst_account_address_keys);
    let pool_reserves = banks_client
        .get_account(pool_reserves_addr)
        .await
        .unwrap()
        .unwrap();
    let pool_reserves_token_account =
        spl_token::state::Account::unpack(&pool_reserves.data).unwrap();
    assert_eq!(pool_reserves_token_account.owner, POOL_STATE_ID);
    assert_eq!(pool_reserves_token_account.amount, 0);

    let (protocol_fee_accumulator_addr, protocol_fee_accumulator_bump) =
        find_protocol_fee_accumulator_address(find_lst_account_address_keys);
    let protocol_fee_accumulator = banks_client
        .get_account(protocol_fee_accumulator_addr)
        .await
        .unwrap()
        .unwrap();
    let protocol_fee_accumulator_token_account =
        spl_token::state::Account::unpack(&protocol_fee_accumulator.data).unwrap();
    assert_eq!(
        protocol_fee_accumulator_token_account.owner,
        PROTOCOL_FEE_ID
    );
    assert_eq!(protocol_fee_accumulator_token_account.amount, 0);

    let (i, lst_state) =
        try_find_lst_mint_on_list(find_lst_account_address_keys.lst_mint, lst_state_list).unwrap();

    assert_eq!(i, expected_index);
    assert_eq!(lst_state.pool_reserves_bump, pool_reserves_bump);
    assert_eq!(
        lst_state.protocol_fee_accumulator_bump,
        protocol_fee_accumulator_bump
    );
    assert_eq!(lst_state.sol_value, 0);
    assert_eq!(
        lst_state.sol_value_calculator,
        expected_sol_value_calculator
    );
}
