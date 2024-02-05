use s_controller_interface::{initialize_ix, PoolState};
use s_controller_lib::{
    initial_authority, program::POOL_STATE_ID, try_pool_state, InitializeFreeArgs,
    CURRENT_PROGRAM_VERS, DEFAULT_LP_PROTOCOL_FEE_BPS, DEFAULT_PRICING_PROGRAM,
    DEFAULT_TRADING_PROTOCOL_FEE_BPS,
};
use s_controller_test_utils::{LpTokenProgramTest, MockLpMintToInitArgs, PoolStateBanksClient};
use sanctum_solana_test_utils::{
    assert_built_in_prog_err, assert_program_error, test_fixtures_dir, ExtendedBanksClient,
};
use solana_program::{
    program_error::ProgramError,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction::{self, SystemError},
};
use solana_program_test::ProgramTest;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};
use spl_token::{native_mint, state::Mint};

use crate::common::*;

/// Returns (program_test, lp_token_mint_addr)
fn setup(lp_mint_intial_auth: Pubkey) -> (ProgramTest, Pubkey) {
    let lp_token_mint_addr = Pubkey::new_unique();
    let program_test = ProgramTest::default()
        .add_s_controller_prog()
        .add_mock_lp_mint_to_init(MockLpMintToInitArgs {
            initial_authority: lp_mint_intial_auth,
            addr: lp_token_mint_addr,
        });
    (program_test, lp_token_mint_addr)
}

#[tokio::test]
async fn initialize_basic() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let (program_test, lp_token_mint_addr) = setup(mock_auth_kp.pubkey());
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = initialize_ix(
        InitializeFreeArgs {
            payer: payer.pubkey(),
            lp_token_mint: lp_token_mint_addr,
        }
        .resolve(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let pool_state_acc = banks_client.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert_eq!(
        *pool_state,
        PoolState {
            total_sol_value: 0,
            trading_protocol_fee_bps: DEFAULT_TRADING_PROTOCOL_FEE_BPS,
            lp_protocol_fee_bps: DEFAULT_LP_PROTOCOL_FEE_BPS,
            version: CURRENT_PROGRAM_VERS,
            is_disabled: 0,
            is_rebalancing: 0,
            admin: initial_authority::ID,
            rebalance_authority: initial_authority::ID,
            protocol_fee_beneficiary: initial_authority::ID,
            pricing_program: DEFAULT_PRICING_PROGRAM,
            lp_token_mint: lp_token_mint_addr,
            padding: [0; 1] // dont care
        }
    );

    let lp_token_mint_acc = banks_client.get_account_unwrapped(lp_token_mint_addr).await;
    let lp_token_mint = Mint::unpack(&lp_token_mint_acc.data).unwrap();
    assert_eq!(
        lp_token_mint,
        Mint {
            mint_authority: COption::Some(POOL_STATE_ID),
            supply: 0,
            decimals: native_mint::DECIMALS,
            is_initialized: true,
            freeze_authority: COption::Some(POOL_STATE_ID),
        }
    );
}

#[tokio::test]
async fn fail_init_unauthorized() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let (program_test, lp_token_mint_addr) = setup(mock_auth_kp.pubkey());
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let mut ix = initialize_ix(
        InitializeFreeArgs {
            payer: payer.pubkey(),
            lp_token_mint: lp_token_mint_addr,
        }
        .resolve(),
    )
    .unwrap();
    ix.accounts[1].pubkey = payer.pubkey();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_program_error(err, ProgramError::InvalidArgument);
}

#[tokio::test]
async fn fail_init_second_time() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let (program_test, lp_token_mint_addr) = setup(mock_auth_kp.pubkey());
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = initialize_ix(
        InitializeFreeArgs {
            payer: payer.pubkey(),
            lp_token_mint: lp_token_mint_addr,
        }
        .resolve(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix.clone()], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    // Must change the transaction else duplicate and previous Ok(()) result will be returned
    let dummy_transfer_ix = system_instruction::transfer(&payer.pubkey(), &payer.pubkey(), 1);
    let mut tx = Transaction::new_with_payer(&[ix, dummy_transfer_ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_built_in_prog_err(err, SystemError::AccountAlreadyInUse);
}
