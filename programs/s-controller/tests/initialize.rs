use s_controller_interface::{initialize_ix, InitializeIxArgs, PoolState};
use s_controller_lib::{
    initial_authority, program::POOL_STATE_ID, try_pool_state, InitializeFreeArgs,
    CURRENT_PROGRAM_VERS, DEFAULT_LP_PROTOCOL_FEE_BPS, DEFAULT_PRICING_PROGRAM,
    DEFAULT_TRADING_PROTOCOL_FEE_BPS,
};
use solana_program::{program_option::COption, program_pack::Pack};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use spl_token_2022::{native_mint, state::Mint};
use test_utils::test_fixtures_dir;

mod common;

use crate::common::*;

#[tokio::test]
async fn basic() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let lp_token_mint_kp = Keypair::new();

    let mut program_test = ProgramTest::default();

    // name must match <name>.so filename
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = initialize_ix(
        InitializeFreeArgs {
            payer: payer.pubkey(),
            lp_token_mint: lp_token_mint_kp.pubkey(),
        }
        .resolve(),
        InitializeIxArgs {},
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp, &lp_token_mint_kp], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
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
            lp_token_mint: lp_token_mint_kp.pubkey(),
            padding: [0; 1] // dont care
        }
    );

    let lp_token_mint_acc = banks_client
        .get_account(lp_token_mint_kp.pubkey())
        .await
        .unwrap()
        .unwrap();
    let lp_token_mint = Mint::unpack_from_slice(&lp_token_mint_acc.data).unwrap();
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
