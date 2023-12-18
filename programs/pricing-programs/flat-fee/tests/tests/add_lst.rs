use flat_fee_interface::{add_lst_ix, AddLstIxArgs, AddLstKeys, FlatFeeError, ProgramState};
use flat_fee_lib::{
    account_resolvers::AddLstFreeArgs, pda::FeeAccountFindPdaArgs, program::STATE_ID,
};
use flat_fee_test_utils::FlatFeePricingProgramTestBanksClient;
use sanctum_solana_test_utils::{assert_custom_err, assert_program_error};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::{normal_program_test, verify_fee_account, verify_fee_account_does_not_exist};

#[tokio::test]
async fn add_lst_basic() {
    const FEE_ARGS: AddLstIxArgs = AddLstIxArgs {
        input_fee_bps: 6,
        output_fee_bps: 9,
    };

    let manager = Keypair::new();
    let lst_mint = Pubkey::new_unique();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[],
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    verify_fee_account_does_not_exist(&mut banks_client, lst_mint).await;
    let state_acc = banks_client.get_flat_fee_program_state().await;

    let free_args = AddLstFreeArgs {
        payer: payer.pubkey(),
        lst_mint,
        state_acc: KeyedAccount {
            pubkey: STATE_ID,
            account: state_acc,
        },
    };
    let (keys, _pda) = free_args.resolve().unwrap();
    let ix = add_lst_ix(keys, FEE_ARGS).unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    verify_fee_account(&mut banks_client, lst_mint, FEE_ARGS).await;
}

#[tokio::test]
async fn add_lst_fail_invalid_fee() {
    const BAD_FEE_ARGS_1: AddLstIxArgs = AddLstIxArgs {
        input_fee_bps: 10_001,
        output_fee_bps: 0,
    };
    const BAD_FEE_ARGS_2: AddLstIxArgs = AddLstIxArgs {
        input_fee_bps: 0,
        output_fee_bps: 10_001,
    };

    let manager = Keypair::new();
    let lst_mint = Pubkey::new_unique();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[],
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    verify_fee_account_does_not_exist(&mut banks_client, lst_mint).await;
    let state_acc = banks_client.get_flat_fee_program_state().await;
    let keyed_state_acc = KeyedAccount {
        pubkey: STATE_ID,
        account: state_acc,
    };

    for bad_fee_args in [BAD_FEE_ARGS_1, BAD_FEE_ARGS_2] {
        let free_args = AddLstFreeArgs {
            payer: payer.pubkey(),
            lst_mint,
            state_acc: &keyed_state_acc,
        };
        let (keys, _pda) = free_args.resolve().unwrap();
        let ix = add_lst_ix(keys, bad_fee_args).unwrap();

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &manager], last_blockhash);

        let err = banks_client.process_transaction(tx).await.unwrap_err();
        assert_custom_err(err, FlatFeeError::SignedFeeOutOfBound);

        verify_fee_account_does_not_exist(&mut banks_client, lst_mint).await;
    }
}

#[tokio::test]
async fn add_lst_fail_unauthorized() {
    let manager = Keypair::new();
    let lst_mint = Pubkey::new_unique();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[],
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    verify_fee_account_does_not_exist(&mut banks_client, lst_mint).await;

    let find_pda_args = FeeAccountFindPdaArgs { lst_mint };
    let (fee_acc, _bump) = find_pda_args.get_fee_account_address_and_bump_seed();

    let ix = add_lst_ix(
        AddLstKeys {
            manager: payer.pubkey(),
            payer: payer.pubkey(),
            fee_acc,
            lst_mint,
            state: STATE_ID,
            system_program: solana_program::system_program::ID,
        },
        AddLstIxArgs {
            input_fee_bps: 0,
            output_fee_bps: 0,
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_program_error(err, ProgramError::InvalidArgument);

    verify_fee_account_does_not_exist(&mut banks_client, lst_mint).await;
}
