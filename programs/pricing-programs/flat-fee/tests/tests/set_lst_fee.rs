use flat_fee_interface::{
    set_lst_fee_ix, AddLstIxArgs, FlatFeeError, ProgramState, SetLstFeeIxArgs,
};
use flat_fee_lib::{
    account_resolvers::SetLstFeeByMintFreeArgs, pda::FeeAccountFindPdaArgs, program::STATE_ID,
    utils::try_fee_account,
};
use flat_fee_test_utils::MockFeeAccountArgs;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use test_utils::{assert_custom_err, assert_program_error, banks_client_get_account};

use crate::common::*;

#[tokio::test]
async fn set_lst_fee_basic() {
    const FEE_ARGS: AddLstIxArgs = AddLstIxArgs {
        input_fee_bps: -2,
        output_fee_bps: 10000,
    };

    let manager = Keypair::new();
    let lst_mint = Pubkey::new_unique();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[MockFeeAccountArgs {
            input_fee_bps: Default::default(),
            output_fee_bps: Default::default(),
            lst_mint,
        }],
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let state_acc = banks_client_get_account(&mut banks_client, STATE_ID).await;
    let ix = set_lst_fee_ix(
        SetLstFeeByMintFreeArgs {
            lst_mint,
            state_acc: KeyedAccount {
                pubkey: STATE_ID,
                account: state_acc,
            },
        }
        .resolve()
        .unwrap(),
        SetLstFeeIxArgs {
            input_fee_bps: FEE_ARGS.input_fee_bps,
            output_fee_bps: FEE_ARGS.output_fee_bps,
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    verify_fee_account(&mut banks_client, lst_mint, FEE_ARGS).await;
}

#[tokio::test]
async fn set_lst_fee_fail_invalid_fee() {
    const BAD_FEE_ARGS_1: AddLstIxArgs = AddLstIxArgs {
        input_fee_bps: 10_001,
        output_fee_bps: 6,
    };
    const BAD_FEE_ARGS_2: AddLstIxArgs = AddLstIxArgs {
        input_fee_bps: 9,
        output_fee_bps: 10_001,
    };

    let manager = Keypair::new();
    let lst_mint = Pubkey::new_unique();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[MockFeeAccountArgs {
            input_fee_bps: Default::default(),
            output_fee_bps: Default::default(),
            lst_mint,
        }],
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let state_acc = banks_client_get_account(&mut banks_client, STATE_ID).await;
    let keyed_state_acc = KeyedAccount {
        pubkey: STATE_ID,
        account: state_acc,
    };
    let (fee_account_pk, _bump) =
        FeeAccountFindPdaArgs { lst_mint }.get_fee_account_address_and_bump_seed();

    for bad_fee_args in [BAD_FEE_ARGS_1, BAD_FEE_ARGS_2] {
        let ix = set_lst_fee_ix(
            SetLstFeeByMintFreeArgs {
                lst_mint,
                state_acc: &keyed_state_acc,
            }
            .resolve()
            .unwrap(),
            SetLstFeeIxArgs {
                input_fee_bps: bad_fee_args.input_fee_bps,
                output_fee_bps: bad_fee_args.output_fee_bps,
            },
        )
        .unwrap();

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &manager], last_blockhash);

        let err = banks_client.process_transaction(tx).await.unwrap_err();

        assert_custom_err(err, FlatFeeError::SignedFeeOutOfBound);

        let fee_account_acc = banks_client_get_account(&mut banks_client, fee_account_pk).await;
        let fee_account = try_fee_account(&fee_account_acc.data).unwrap();

        assert_ne!(fee_account.input_fee_bps, bad_fee_args.input_fee_bps);
        assert_ne!(fee_account.output_fee_bps, bad_fee_args.output_fee_bps);
        verify_fee_account(
            &mut banks_client,
            lst_mint,
            AddLstIxArgs {
                input_fee_bps: Default::default(),
                output_fee_bps: Default::default(),
            },
        )
        .await;
    }
}

#[tokio::test]
async fn set_lst_fee_fail_unauthorized() {
    const FEE_ARGS: AddLstIxArgs = AddLstIxArgs {
        input_fee_bps: -2,
        output_fee_bps: 10000,
    };

    let manager = Keypair::new();
    let lst_mint = Pubkey::new_unique();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            lp_withdrawal_fee_bps: Default::default(),
        },
        &[MockFeeAccountArgs {
            input_fee_bps: Default::default(),
            output_fee_bps: Default::default(),
            lst_mint,
        }],
    );
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let state_acc = banks_client_get_account(&mut banks_client, STATE_ID).await;
    let mut keys = SetLstFeeByMintFreeArgs {
        lst_mint,
        state_acc: KeyedAccount {
            pubkey: STATE_ID,
            account: state_acc,
        },
    }
    .resolve()
    .unwrap();
    keys.manager = payer.pubkey();

    let ix = set_lst_fee_ix(
        keys,
        SetLstFeeIxArgs {
            input_fee_bps: FEE_ARGS.input_fee_bps,
            output_fee_bps: FEE_ARGS.output_fee_bps,
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_program_error(err, ProgramError::InvalidArgument);

    let (fee_account_pk, _bump) =
        FeeAccountFindPdaArgs { lst_mint }.get_fee_account_address_and_bump_seed();
    let fee_account_acc = banks_client_get_account(&mut banks_client, fee_account_pk).await;
    let fee_account = try_fee_account(&fee_account_acc.data).unwrap();

    assert_ne!(fee_account.input_fee_bps, FEE_ARGS.input_fee_bps);
    assert_ne!(fee_account.output_fee_bps, FEE_ARGS.output_fee_bps);
    verify_fee_account(
        &mut banks_client,
        lst_mint,
        AddLstIxArgs {
            input_fee_bps: Default::default(),
            output_fee_bps: Default::default(),
        },
    )
    .await;
}

#[tokio::test]
async fn set_lst_fee_fail_fee_account_not_exists() {
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

    let state_acc = banks_client_get_account(&mut banks_client, STATE_ID).await;
    let ix = set_lst_fee_ix(
        SetLstFeeByMintFreeArgs {
            lst_mint,
            state_acc: KeyedAccount {
                pubkey: STATE_ID,
                account: state_acc,
            },
        }
        .resolve()
        .unwrap(),
        SetLstFeeIxArgs {
            input_fee_bps: Default::default(),
            output_fee_bps: Default::default(),
        },
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_custom_err(err, FlatFeeError::UnsupportedLstMint);
}
