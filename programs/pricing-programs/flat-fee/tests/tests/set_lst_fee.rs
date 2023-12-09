use flat_fee_interface::{set_lst_fee_ix, AddLstIxArgs, FlatFeeError, SetLstFeeIxArgs};
use flat_fee_lib::{
    account_resolvers::SetLstFeeByMintFreeArgs, program::STATE_ID, utils::try_fee_account,
};
use flat_fee_test_utils::{MockFeeAccountArgs, DEFAULT_PROGRAM_STATE};
use solana_program::program_error::ProgramError;
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use test_utils::{
    assert_custom_err, assert_program_error, banks_client_get_account, jitosol, test_fixtures_dir,
};

use crate::common::*;

#[tokio::test]
async fn basic() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("flat-fee-test-initial-manager-key.json"))
            .unwrap();

    const INITIAL_FEE_BPS: i16 = 1;
    let mock_fee_account_args = MockFeeAccountArgs {
        input_fee_bps: INITIAL_FEE_BPS,
        output_fee_bps: INITIAL_FEE_BPS,
        lst_mint: jitosol::ID,
    };
    let (_, mock_fee_account_pk) = mock_fee_account_args.to_fee_account_and_addr();
    let program_test = normal_program_test(DEFAULT_PROGRAM_STATE, &[mock_fee_account_args]);

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;
    let state_acc = banks_client_get_account(&mut banks_client, STATE_ID).await;

    let keyed_state = KeyedReadonlyAccount {
        key: STATE_ID,
        account: state_acc,
    };

    // set lst fee
    {
        const NEW_INPUT_FEE_BPS: i16 = -2;
        const NEW_OUTPUT_FEE_BPS: i16 = 10000;

        let ix = set_lst_fee_ix(
            SetLstFeeByMintFreeArgs {
                lst_mint: jitosol::ID,
                state_acc: &keyed_state,
            }
            .resolve()
            .unwrap(),
            SetLstFeeIxArgs {
                input_fee_bps: NEW_INPUT_FEE_BPS,
                output_fee_bps: NEW_OUTPUT_FEE_BPS,
            },
        )
        .unwrap();

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        verify_fee_account(
            &mut banks_client,
            jitosol::ID,
            AddLstIxArgs {
                input_fee_bps: NEW_INPUT_FEE_BPS,
                output_fee_bps: NEW_OUTPUT_FEE_BPS,
            },
        )
        .await;
    }

    // reject out of bound
    {
        const NEW_INPUT_FEE_BPS: i16 = 10001;
        const NEW_OUTPUT_FEE_BPS: i16 = 10;

        let ix = set_lst_fee_ix(
            SetLstFeeByMintFreeArgs {
                lst_mint: jitosol::ID,
                state_acc: &keyed_state,
            }
            .resolve()
            .unwrap(),
            SetLstFeeIxArgs {
                input_fee_bps: NEW_INPUT_FEE_BPS,
                output_fee_bps: NEW_OUTPUT_FEE_BPS,
            },
        )
        .unwrap();

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        let err = banks_client.process_transaction(tx).await.unwrap_err();

        assert_custom_err(err, FlatFeeError::SignedFeeOutOfBound);

        let fee_account_acc =
            banks_client_get_account(&mut banks_client, mock_fee_account_pk).await;
        let fee_account = try_fee_account(&fee_account_acc.data).unwrap();

        assert_ne!(fee_account.input_fee_bps, NEW_INPUT_FEE_BPS);
        assert_ne!(fee_account.output_fee_bps, NEW_OUTPUT_FEE_BPS);
    }

    // reject unauthorized
    {
        const NEW_INPUT_FEE_BPS: i16 = -200;
        const NEW_OUTPUT_FEE_BPS: i16 = -201;

        let rando_kp = Keypair::new();

        let mut keys = SetLstFeeByMintFreeArgs {
            lst_mint: jitosol::ID,
            state_acc: &keyed_state,
        }
        .resolve()
        .unwrap();

        keys.manager = rando_kp.pubkey();

        let ix = set_lst_fee_ix(
            keys,
            SetLstFeeIxArgs {
                input_fee_bps: NEW_INPUT_FEE_BPS,
                output_fee_bps: NEW_OUTPUT_FEE_BPS,
            },
        )
        .unwrap();

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &rando_kp], last_blockhash);

        let err = banks_client.process_transaction(tx).await.unwrap_err();

        assert_program_error(err, ProgramError::InvalidArgument);

        let fee_account_acc =
            banks_client_get_account(&mut banks_client, mock_fee_account_pk).await;
        let fee_account = try_fee_account(&fee_account_acc.data).unwrap();

        assert_ne!(fee_account.input_fee_bps, NEW_INPUT_FEE_BPS);
        assert_ne!(fee_account.output_fee_bps, NEW_OUTPUT_FEE_BPS);
    }
}
