use s_controller_interface::{
    set_protocol_fee_ix, PoolState, SetProtocolFeeIxArgs, SetProtocolFeeKeys,
};
use s_controller_lib::{program::POOL_STATE_ID, try_pool_state, SetProtocolFeeFreeArgs};
use sanctum_solana_test_utils::{assert_program_error, test_fixtures_dir, IntoAccount};
use solana_program::program_error::ProgramError;
use solana_program_test::BanksClient;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};

use crate::common::{
    naked_pool_state_program_test, MockPoolState, PoolStateBanksClient, DEFAULT_POOL_STATE,
};

#[tokio::test]
async fn admin_set_both() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let old_pool_state = DEFAULT_POOL_STATE;
    let program_test = naked_pool_state_program_test(DEFAULT_POOL_STATE);
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let args = SetProtocolFeeIxArgs {
        new_lp_protocol_fee_bps: Some(9999),
        new_trading_protocol_fee_bps: Some(9998),
    };
    let ix = set_protocol_fee_ix(
        SetProtocolFeeFreeArgs {
            pool_state: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: MockPoolState(old_pool_state).into_account(),
            },
        }
        .resolve()
        .unwrap(),
        args.clone(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    verify_set_correct(&mut banks_client, old_pool_state, args).await;
}

#[tokio::test]
async fn unauthorized_signer() {
    let program_test = naked_pool_state_program_test(DEFAULT_POOL_STATE);
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_protocol_fee_ix(
        SetProtocolFeeKeys {
            admin: payer.pubkey(), // payer is unauthorized
            pool_state: POOL_STATE_ID,
        },
        SetProtocolFeeIxArgs {
            new_trading_protocol_fee_bps: None,
            new_lp_protocol_fee_bps: None,
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);
    let err = banks_client.process_transaction(tx).await.unwrap_err();
    // InvalidArgument thrown by mismatch keys in *_verify_account_keys()
    assert_program_error(err, ProgramError::InvalidArgument);
}

// TODO: tests for setting one-by-one to make sure Option works

async fn verify_set_correct(
    banks_client: &mut BanksClient,
    old_pool_state: PoolState,
    SetProtocolFeeIxArgs {
        new_trading_protocol_fee_bps,
        new_lp_protocol_fee_bps,
    }: SetProtocolFeeIxArgs,
) {
    let new_pool_state_acc = banks_client.get_pool_state_acc().await;
    let new_pool_state = try_pool_state(&new_pool_state_acc.data).unwrap();

    let expected_trading_protocol_fee_bps = match new_trading_protocol_fee_bps {
        Some(b) => b,
        None => old_pool_state.trading_protocol_fee_bps,
    };
    assert_eq!(
        new_pool_state.trading_protocol_fee_bps,
        expected_trading_protocol_fee_bps
    );

    let expected_lp_protocol_fee_bps = match new_lp_protocol_fee_bps {
        Some(b) => b,
        None => old_pool_state.lp_protocol_fee_bps,
    };
    assert_eq!(
        new_pool_state.lp_protocol_fee_bps,
        expected_lp_protocol_fee_bps
    );
}
