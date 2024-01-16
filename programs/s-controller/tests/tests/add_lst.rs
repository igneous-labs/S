use marinade_keys::msol;
use s_controller_interface::{add_lst_ix, LstState, SControllerError};
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address,
    program::{POOL_STATE_ID, PROTOCOL_FEE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, AddLstFreeArgs, FindLstPdaAtaKeys,
};
use sanctum_solana_test_utils::{
    assert_custom_err, assert_program_error, test_fixtures_dir,
    token::{tokenkeg::TokenkegProgramTest, MockTokenAccountArgs},
    ExtendedBanksClient, IntoAccount,
};
use solana_program::{
    hash::Hash, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey, system_instruction,
};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use test_utils::jitosol;

use crate::common::*;

fn jito_marinade_program_test() -> (ProgramTest, Keypair) {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );
    program_test = program_test
        .add_spl_progs()
        .add_marinade_progs()
        .add_jito_stake_pool()
        .add_marinade_stake_pool();
    let pool_state_account = MockPoolState(DEFAULT_POOL_STATE).into_account();
    program_test.add_account(s_controller_lib::program::POOL_STATE_ID, pool_state_account);

    (program_test, mock_auth_kp)
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

async fn add_and_verify_success_jitosol(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    last_blockhash: Hash,
    mock_auth_kp: &Keypair,
) {
    let pool_state_account = banks_client.get_pool_state_acc().await;
    let jitosol_mint_acc = banks_client.get_account_unwrapped(jitosol::ID).await;
    let (keys, _bumps) = AddLstFreeArgs {
        payer: payer.pubkey(),
        sol_value_calculator: spl_calculator_lib::program::ID,
        pool_state: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: pool_state_account,
        },
        lst_mint: KeyedAccount {
            pubkey: jitosol::ID,
            account: jitosol_mint_acc,
        },
    }
    .resolve()
    .unwrap();
    let ix = add_lst_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[payer, mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let lst_state_list_acc = banks_client.get_lst_state_list_acc().await;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
    assert_eq!(lst_state_list.len(), 1);
    verify_lst_added_success(
        banks_client,
        lst_state_list,
        FindLstPdaAtaKeys {
            lst_mint: jitosol::ID,
            token_program: spl_token::ID,
        },
        spl_calculator_lib::program::ID,
        0,
    )
    .await;
}

#[tokio::test]
async fn basic_add_two() {
    let (program_test, mock_auth_kp) = jito_marinade_program_test();
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = banks_client.get_pool_state_acc().await;

    // Add jitoSOL

    add_and_verify_success_jitosol(&mut banks_client, &payer, last_blockhash, &mock_auth_kp).await;

    // Add mSOL

    let msol_mint_acc = banks_client.get_account_unwrapped(msol::ID).await;
    let (keys, _bumps) = AddLstFreeArgs {
        payer: payer.pubkey(),
        sol_value_calculator: marinade_calculator_lib::program::ID,
        pool_state: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: pool_state_account,
        },
        lst_mint: KeyedAccount {
            pubkey: msol::ID,
            account: msol_mint_acc,
        },
    }
    .resolve()
    .unwrap();
    let ix = add_lst_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let lst_state_list_acc = banks_client.get_lst_state_list_acc().await;
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

#[tokio::test]
async fn add_with_pre_created_atas() {
    let (program_test, mock_auth_kp) = jito_marinade_program_test();
    let (pool_reserves, _bump) = find_pool_reserves_address(FindLstPdaAtaKeys {
        lst_mint: jitosol::ID,
        token_program: spl_token::ID,
    });
    let (protocol_fee_accum, _bump) = find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
        lst_mint: jitosol::ID,
        token_program: spl_token::ID,
    });
    let program_test = program_test
        .add_tokenkeg_account_from_args(
            pool_reserves,
            MockTokenAccountArgs {
                mint: jitosol::ID,
                amount: 0,
                authority: POOL_STATE_ID,
            },
        )
        .add_tokenkeg_account_from_args(
            protocol_fee_accum,
            MockTokenAccountArgs {
                mint: jitosol::ID,
                amount: 0,
                authority: PROTOCOL_FEE_ID,
            },
        );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;
    add_and_verify_success_jitosol(&mut banks_client, &payer, last_blockhash, &mock_auth_kp).await;
}

#[tokio::test]
async fn fail_add_duplicate() {
    let (program_test, mock_auth_kp) = jito_marinade_program_test();
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = banks_client.get_pool_state_acc().await;

    // Add jitoSOL
    add_and_verify_success_jitosol(&mut banks_client, &payer, last_blockhash, &mock_auth_kp).await;

    // Add jitoSOL again
    let jitosol_mint_acc = banks_client.get_account_unwrapped(jitosol::ID).await;
    let (keys, _bumps) = AddLstFreeArgs {
        payer: payer.pubkey(),
        sol_value_calculator: spl_calculator_lib::program::ID,
        pool_state: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: pool_state_account,
        },
        lst_mint: KeyedAccount {
            pubkey: jitosol::ID,
            account: jitosol_mint_acc,
        },
    }
    .resolve()
    .unwrap();
    let ix = add_lst_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(
        &[
            ix,
            // modify tx so that its not treated as a replay
            system_instruction::transfer(&payer.pubkey(), &payer.pubkey(), 1),
        ],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_custom_err(err, SControllerError::DuplicateLst);
}

// should never happen
#[tokio::test]
async fn fail_pre_created_ata_wrong_authority() {
    let (program_test, mock_auth_kp) = jito_marinade_program_test();
    let (pool_reserves, _bump) = find_pool_reserves_address(FindLstPdaAtaKeys {
        lst_mint: jitosol::ID,
        token_program: spl_token::ID,
    });
    let program_test = program_test.add_tokenkeg_account_from_args(
        pool_reserves,
        MockTokenAccountArgs {
            mint: jitosol::ID,
            amount: 0,
            authority: Pubkey::new_unique(),
        },
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let jitosol_mint_acc = banks_client.get_account_unwrapped(jitosol::ID).await;
    let (keys, _bumps) = AddLstFreeArgs {
        payer: payer.pubkey(),
        sol_value_calculator: spl_calculator_lib::program::ID,
        pool_state: KeyedAccount {
            pubkey: POOL_STATE_ID,
            account: pool_state_account,
        },
        lst_mint: KeyedAccount {
            pubkey: jitosol::ID,
            account: jitosol_mint_acc,
        },
    }
    .resolve()
    .unwrap();
    let ix = add_lst_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_program_error(err, ProgramError::InvalidAccountData);
}
