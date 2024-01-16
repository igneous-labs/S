use flat_fee_interface::{add_lst_ix, AddLstIxArgs, AddLstKeys, FlatFeeError, ProgramState};
use flat_fee_lib::{
    account_resolvers::AddLstFreeArgs, pda::FeeAccountFindPdaArgs, program::STATE_ID,
};
use flat_fee_test_utils::FlatFeePricingProgramTestBanksClient;
use sanctum_solana_test_utils::{
    assert_custom_err, assert_program_error,
    token::{tokenkeg::TokenkegProgramTest, MockMintArgs},
};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_program_test::ProgramTest;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::{normal_program_test, verify_fee_account, verify_fee_account_does_not_exist};

fn add_lst_program_test() -> (ProgramTest, Keypair, Pubkey) {
    let manager = Keypair::new();
    let lst_mint = Pubkey::new_unique();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[],
    )
    .add_tokenkeg_mint_from_args(
        lst_mint,
        MockMintArgs {
            mint_authority: None,
            freeze_authority: None,
            supply: 0,
            decimals: 9,
        },
    );
    (program_test, manager, lst_mint)
}

#[tokio::test]
async fn add_lst_basic() {
    const FEE_ARGS: AddLstIxArgs = AddLstIxArgs {
        input_fee_bps: 6,
        output_fee_bps: 9,
    };

    let (program_test, manager, lst_mint) = add_lst_program_test();
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
    const BAD_FEE_ARGS: [AddLstIxArgs; 4] = [
        AddLstIxArgs {
            input_fee_bps: 10_001,
            output_fee_bps: 0,
        },
        AddLstIxArgs {
            input_fee_bps: 0,
            output_fee_bps: 10_001,
        },
        AddLstIxArgs {
            input_fee_bps: -10_001,
            output_fee_bps: 0,
        },
        AddLstIxArgs {
            input_fee_bps: 0,
            output_fee_bps: -10_001,
        },
    ];

    let (program_test, manager, lst_mint) = add_lst_program_test();
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    verify_fee_account_does_not_exist(&mut banks_client, lst_mint).await;
    let state_acc = banks_client.get_flat_fee_program_state().await;
    let keyed_state_acc = KeyedAccount {
        pubkey: STATE_ID,
        account: state_acc,
    };

    for bad_fee_args in BAD_FEE_ARGS {
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
    let (program_test, _manager, lst_mint) = add_lst_program_test();
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

#[tokio::test]
async fn add_lst_fail_invalid_lst_mint() {
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
            manager: manager.pubkey(),
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
    tx.sign(&[&payer, &manager], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_program_error(err, ProgramError::IllegalOwner);

    verify_fee_account_does_not_exist(&mut banks_client, lst_mint).await;
}
