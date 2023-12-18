use flat_fee_interface::{set_manager_ix, ProgramState};
use flat_fee_lib::{
    account_resolvers::SetManagerFreeArgs, program::STATE_ID, utils::try_program_state,
};
use flat_fee_test_utils::FlatFeePricingProgramTestBanksClient;
use sanctum_solana_test_utils::{assert_program_error, ExtendedBanksClient};
use solana_program::program_error::ProgramError;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::normal_program_test;

#[tokio::test]
async fn set_manager_basic() {
    let new_manager = Keypair::new();

    let manager = Keypair::new();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[],
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let state_acc = banks_client.get_account_unwrapped(STATE_ID).await;
    let ix = set_manager_ix(
        SetManagerFreeArgs {
            new_manager: new_manager.pubkey(),
            state_acc: KeyedAccount {
                pubkey: STATE_ID,
                account: state_acc,
            },
        }
        .resolve()
        .unwrap(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    let state_acc = banks_client.get_flat_fee_program_state().await;
    let state = try_program_state(&state_acc.data).unwrap();

    assert_eq!(state.manager, new_manager.pubkey());
}

#[tokio::test]
async fn set_manager_fail_unauthorized() {
    let new_manager = Keypair::new();

    let manager = Keypair::new();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[],
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let state_acc = banks_client.get_account_unwrapped(STATE_ID).await;
    let mut keys = SetManagerFreeArgs {
        new_manager: new_manager.pubkey(),
        state_acc: KeyedAccount {
            pubkey: STATE_ID,
            account: state_acc,
        },
    }
    .resolve()
    .unwrap();
    keys.current_manager = payer.pubkey();

    let ix = set_manager_ix(keys).unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_program_error(err, ProgramError::InvalidArgument);

    let state_acc = banks_client.get_flat_fee_program_state().await;
    let state = try_program_state(&state_acc.data).unwrap();

    assert_eq!(state.manager, manager.pubkey());
}
