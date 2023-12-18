// use flat_fee_interface::{remove_lst_ix, AddLstIxArgs, ProgramState, RemoveLstIxArgs};
use flat_fee_interface::{remove_lst_ix, AddLstIxArgs, ProgramState};
use flat_fee_lib::{account_resolvers::RemoveLstByMintFreeArgs, program::STATE_ID};
use flat_fee_test_utils::{MockFeeAccountArgs, DEFAULT_PROGRAM_STATE};
use sanctum_solana_test_utils::{assert_program_error, ExtendedBanksClient};
use solana_program::program_error::ProgramError;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use test_utils::jitosol;

use crate::common::{normal_program_test, verify_fee_account, verify_fee_account_does_not_exist};

#[tokio::test]
async fn remove_lst_basic() {
    let manager = Keypair::new();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            ..DEFAULT_PROGRAM_STATE
        },
        &[MockFeeAccountArgs {
            input_fee_bps: 1,
            output_fee_bps: 2,
            lst_mint: jitosol::ID,
        }],
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let free_args = RemoveLstByMintFreeArgs {
        refund_rent_to: payer.pubkey(),
        lst_mint: jitosol::ID,
        state_acc: KeyedAccount {
            pubkey: STATE_ID,
            account: banks_client.get_account_unwrapped(STATE_ID).await,
        },
    };
    let ix = remove_lst_ix(free_args.resolve().unwrap()).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], last_blockhash);

    banks_client.process_transaction(tx).await.unwrap();

    verify_fee_account_does_not_exist(&mut banks_client, jitosol::ID).await;
}

#[tokio::test]
async fn remove_lst_fail_unauthorized() {
    const MOCK_FEE_ACCOUNT_ARGS: MockFeeAccountArgs = MockFeeAccountArgs {
        input_fee_bps: 6,
        output_fee_bps: 9,
        lst_mint: jitosol::ID,
    };
    let manager = Keypair::new();

    let program_test = normal_program_test(
        ProgramState {
            manager: manager.pubkey(),
            ..DEFAULT_PROGRAM_STATE
        },
        &[MOCK_FEE_ACCOUNT_ARGS],
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let mut keys = RemoveLstByMintFreeArgs {
        refund_rent_to: payer.pubkey(),
        lst_mint: jitosol::ID,
        state_acc: KeyedAccount {
            pubkey: STATE_ID,
            account: banks_client.get_account_unwrapped(STATE_ID).await,
        },
    }
    .resolve()
    .unwrap();
    keys.manager = payer.pubkey();
    let ix = remove_lst_ix(keys).unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_program_error(err, ProgramError::InvalidArgument);

    verify_fee_account(
        &mut banks_client,
        jitosol::ID,
        AddLstIxArgs {
            input_fee_bps: MOCK_FEE_ACCOUNT_ARGS.input_fee_bps,
            output_fee_bps: MOCK_FEE_ACCOUNT_ARGS.output_fee_bps,
        },
    )
    .await;
}
