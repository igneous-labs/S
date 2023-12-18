use flat_fee_interface::{
    set_lp_withdrawal_fee_ix, ProgramState, SetLpWithdrawalFeeIxArgs, SetLpWithdrawalFeeKeys,
};
use flat_fee_lib::{
    account_resolvers::SetLpWithdrawalFeeFreeArgs, program::STATE_ID, utils::try_program_state,
};
use flat_fee_test_utils::FlatFeePricingProgramTestBanksClient;
use sanctum_solana_test_utils::assert_program_error;
use solana_program::program_error::ProgramError;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::normal_program_test;

#[tokio::test]
async fn set_lp_withdrawal_fee_basic() {
    const NEW_LP_WITHDRAWAL_FEE_BPS: u16 = 420;
    let manager = Keypair::new();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[],
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let state_acc = banks_client.get_flat_fee_program_state().await;
    let ix = set_lp_withdrawal_fee_ix(
        SetLpWithdrawalFeeFreeArgs {
            state_acc: KeyedAccount {
                pubkey: STATE_ID,
                account: state_acc,
            },
        }
        .resolve()
        .unwrap(),
        SetLpWithdrawalFeeIxArgs {
            lp_withdrawal_fee_bps: NEW_LP_WITHDRAWAL_FEE_BPS,
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let state_acc = banks_client.get_flat_fee_program_state().await;
    let state = try_program_state(&state_acc.data).unwrap();

    assert_eq!(state.lp_withdrawal_fee_bps, NEW_LP_WITHDRAWAL_FEE_BPS);
}

#[tokio::test]
async fn set_lp_withdrawal_fee_fail_unauthorized() {
    const NEW_LP_WITHDRAWAL_FEE_BPS: u16 = 69;
    let manager = Keypair::new();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[],
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_lp_withdrawal_fee_ix(
        SetLpWithdrawalFeeKeys {
            manager: payer.pubkey(),
            state: STATE_ID,
        },
        SetLpWithdrawalFeeIxArgs {
            lp_withdrawal_fee_bps: NEW_LP_WITHDRAWAL_FEE_BPS,
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_program_error(err, ProgramError::InvalidArgument);

    let state_acc = banks_client.get_flat_fee_program_state().await;
    let state = try_program_state(&state_acc.data).unwrap();

    assert_ne!(state.lp_withdrawal_fee_bps, NEW_LP_WITHDRAWAL_FEE_BPS);
}
