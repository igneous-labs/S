use flat_fee_interface::{set_manager_ix, SetManagerIxArgs};
use flat_fee_lib::{
    account_resolvers::SetManagerFreeArgs, program::STATE_ID, utils::try_program_state,
};
use flat_fee_test_utils::{banks_client_get_flat_fee_program_state, DEFAULT_PROGRAM_STATE};
use solana_program::program_error::ProgramError;
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedReadonlyAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use test_utils::{assert_program_error, banks_client_get_account, test_fixtures_dir};

use crate::common::normal_program_test;

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

    let program_test = normal_program_test(DEFAULT_PROGRAM_STATE, &[]);

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    // set admin success
    {
        let state_acc = banks_client_get_account(&mut banks_client, STATE_ID).await;
        let new_manager_kp = Keypair::new();
        let ix = set_manager_ix(
            SetManagerFreeArgs {
                new_manager: new_manager_kp.pubkey(),
                state_acc: KeyedReadonlyAccount {
                    key: STATE_ID,
                    account: state_acc,
                },
            }
            .resolve()
            .unwrap(),
            SetManagerIxArgs {},
        )
        .unwrap();

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let state_acc = banks_client_get_flat_fee_program_state(&mut banks_client).await;
        let state = try_program_state(&state_acc.data).unwrap();

        assert_eq!(state.manager, new_manager_kp.pubkey());
    }

    // set admin rejection
    {
        let state_acc = banks_client_get_flat_fee_program_state(&mut banks_client).await;
        let state = try_program_state(&state_acc.data).unwrap();
        let original_manager = state.manager;

        // A keypair not authorized to set manager
        let rando_kp = Keypair::new();
        let new_manager_kp = Keypair::new();
        let mut keys = SetManagerFreeArgs {
            new_manager: new_manager_kp.pubkey(),
            state_acc: KeyedReadonlyAccount {
                key: STATE_ID,
                account: state_acc,
            },
        }
        .resolve()
        .unwrap();
        keys.current_manager = rando_kp.pubkey();

        let ix = set_manager_ix(keys, SetManagerIxArgs {}).unwrap();

        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &rando_kp], last_blockhash);

        let err = banks_client.process_transaction(tx).await.unwrap_err();

        assert_program_error(err, ProgramError::InvalidArgument);

        let state_acc = banks_client_get_flat_fee_program_state(&mut banks_client).await;
        let state = try_program_state(&state_acc.data).unwrap();

        assert_eq!(state.manager, original_manager);
    }
}
