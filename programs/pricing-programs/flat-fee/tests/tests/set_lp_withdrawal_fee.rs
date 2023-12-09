use flat_fee_interface::{set_lp_withdrawal_fee_ix, SetLpWithdrawalFeeIxArgs};
use flat_fee_lib::{
    account_resolvers::SetLpWithdrawalFeeFreeArgs, program::STATE_ID, utils::try_program_state,
};
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};
use test_utils::test_fixtures_dir;

use crate::common::*;

#[tokio::test]
async fn basic() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("flat-fee-test-initial-manager-key.json"))
            .unwrap();

    let mut program_test = ProgramTest::default();

    program_test.add_program(
        "flat_fee",
        flat_fee_lib::program::ID,
        processor!(flat_fee::entrypoint::process_instruction),
    );

    let program_state_acc = program_state_to_account(DEFAULT_PROGRAM_STATE);
    program_test.add_account(flat_fee_lib::program::STATE_ID, program_state_acc.clone());

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // set lp withdrawal fee
    {
        let new_lp_withdrawal_fee_bps = 100;
        let ix = set_lp_withdrawal_fee_ix(
            SetLpWithdrawalFeeFreeArgs {
                state_acc: KeyedReadonlyAccount {
                    key: STATE_ID,
                    account: program_state_acc.clone(),
                },
            }
            .resolve()
            .unwrap(),
            SetLpWithdrawalFeeIxArgs {
                lp_withdrawal_fee_bps: new_lp_withdrawal_fee_bps,
            },
        )
        .unwrap();

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let state_acc = banks_client_get_program_state_acc(&mut banks_client).await;
        let state = try_program_state(&state_acc.data).unwrap();

        assert_eq!(state.lp_withdrawal_fee_bps, new_lp_withdrawal_fee_bps);
    }
}
