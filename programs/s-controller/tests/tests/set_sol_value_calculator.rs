use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use marinade_keys::msol;
use s_controller_interface::{SControllerError, SetSolValueCalculatorKeys};
use s_controller_lib::{
    create_pool_reserves_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    set_sol_value_calculator_ix_by_mint_full, set_sol_value_calculator_ix_full,
    try_find_lst_mint_on_list, try_lst_state_list, try_pool_state,
    SetSolValueCalculatorByMintFreeArgs,
};
use s_controller_test_utils::{
    jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs, LstStateListBanksClient,
    LstStateListProgramTest, MockLstStateArgs, PoolStateBanksClient,
};
use sanctum_solana_test_utils::{assert_custom_err, assert_program_error, test_fixtures_dir};
use sanctum_token_lib::MintWithTokenProgram;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};

use crate::common::SControllerProgramTest;

#[tokio::test]
async fn basic_set_marinade() {
    const MSOL_POOL_RESERVES: u64 = 1_000_000_000;

    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        // these are overriden below
        msol_reserves: MSOL_POOL_RESERVES,
        msol_sol_value: MSOL_POOL_RESERVES,
        // dont cares
        jitosol_reserves: 0,
        jitosol_sol_value: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_mock_lst_states(
        // set mSOL initial calculator to a broken pubkey
        &[MockLstStateArgs {
            mint: msol::ID,
            sol_value_calculator: Pubkey::new_unique(),
            token_program: spl_token::ID,
            sol_value: MSOL_POOL_RESERVES,
            reserves_amt: MSOL_POOL_RESERVES,
            protocol_fee_accumulator_amt: 0,
        }],
    )
    .add_s_program();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;
    let lst_state_list = try_lst_state_list(&lst_state_list_account.data).unwrap();
    let (_i, lst_state) = try_find_lst_mint_on_list(msol::ID, lst_state_list).unwrap();
    assert!(lst_state.sol_value_calculator != marinade_calculator_lib::program::ID);

    let ix = set_sol_value_calculator_ix_by_mint_full(
        &SetSolValueCalculatorByMintFreeArgs {
            pool_state: banks_client.get_pool_state_acc().await,
            lst_state_list: lst_state_list_account,
            lst_mint: MintWithTokenProgram {
                pubkey: msol::ID,
                token_program: spl_token::ID,
            },
        },
        &marinade_sol_val_calc_account_metas(),
        marinade_calculator_lib::program::ID,
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;
    let lst_state_list = try_lst_state_list(&lst_state_list_account.data).unwrap();
    let (_i, lst_state) = try_find_lst_mint_on_list(msol::ID, lst_state_list).unwrap();
    assert_eq!(
        lst_state.sol_value_calculator,
        marinade_calculator_lib::program::ID
    );
    // should have increased to true rate after sync
    assert!(lst_state.sol_value > MSOL_POOL_RESERVES);

    let pool_state_account = banks_client.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_account.data).unwrap();
    // should have increased to true rate after sync
    assert!(pool_state.total_sol_value > MSOL_POOL_RESERVES);
}

#[tokio::test]
async fn fail_unauthorized() {
    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        // dont cares
        msol_reserves: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        jitosol_sol_value: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_s_program();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let lst_state_list_account = banks_client.get_lst_state_list_acc().await;
    let (lst_index, lst_state) = try_find_lst_mint_on_list(
        msol::ID,
        try_lst_state_list(&lst_state_list_account.data).unwrap(),
    )
    .unwrap();
    let pool_reserves = create_pool_reserves_address(lst_state, spl_token::ID).unwrap();

    let ix = set_sol_value_calculator_ix_full(
        SetSolValueCalculatorKeys {
            admin: payer.pubkey(), // payer is unauthorized
            lst_mint: msol::ID,
            pool_state: POOL_STATE_ID,
            pool_reserves,
            lst_state_list: LST_STATE_LIST_ID,
        },
        lst_index,
        &marinade_sol_val_calc_account_metas(),
        marinade_calculator_lib::program::ID,
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    // InvalidArgument thrown by mismatch keys in *_verify_account_keys()
    assert_program_error(err, ProgramError::InvalidArgument);
}

#[tokio::test]
async fn fail_set_non_exec_sol_val_calc() {
    const MSOL_POOL_RESERVES: u64 = 1_000_000_000;

    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let program_test = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        // these are overriden below
        msol_reserves: MSOL_POOL_RESERVES,
        msol_sol_value: MSOL_POOL_RESERVES,
        // dont cares
        jitosol_reserves: 0,
        jitosol_sol_value: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_mock_lst_states(&[MockLstStateArgs {
        mint: msol::ID,
        sol_value_calculator: marinade_calculator_lib::program::ID,
        token_program: spl_token::ID,
        sol_value: MSOL_POOL_RESERVES,
        reserves_amt: MSOL_POOL_RESERVES,
        protocol_fee_accumulator_amt: 0,
    }])
    .add_s_program();

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let uninitialized_sol_val_calc_program = Pubkey::new_unique();
    let ix = set_sol_value_calculator_ix_by_mint_full(
        &SetSolValueCalculatorByMintFreeArgs {
            pool_state: banks_client.get_pool_state_acc().await,
            lst_state_list: banks_client.get_lst_state_list_acc().await,
            lst_mint: MintWithTokenProgram {
                pubkey: msol::ID,
                token_program: spl_token::ID,
            },
        },
        &[],
        uninitialized_sol_val_calc_program,
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_custom_err(err, SControllerError::FaultySolValueCalculator);
}
