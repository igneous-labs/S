use s_controller_interface::{set_pricing_program_ix, SetPricingProgramIxArgs};
use s_controller_lib::{
    program::POOL_STATE_ID, try_pool_state, SetPricingProgramFreeArgs, DEFAULT_PRICING_PROGRAM,
};

use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use test_utils::{assert_program_error, test_fixtures_dir};

use crate::common::*;

#[tokio::test]
async fn basic() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();

    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );

    let pool_state_account = pool_state_to_account(DEFAULT_POOL_STATE);
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        pool_state_account.clone(),
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    const TEST_PRICING_PROGRAM_SUCCESS: Pubkey = Pubkey::new_from_array([1; 32]);
    const TEST_PRICING_PROGRAM_FAILURE: Pubkey = Pubkey::new_from_array([2; 32]);

    // Basic success case
    {
        assert_ne!(TEST_PRICING_PROGRAM_SUCCESS, DEFAULT_PRICING_PROGRAM);
        let keys = SetPricingProgramFreeArgs {
            new_pricing_program: TEST_PRICING_PROGRAM_SUCCESS,
            pool_state_acc: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
        }
        .resolve()
        .unwrap();
        let ix = set_pricing_program_ix(keys, SetPricingProgramIxArgs {}).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &mock_auth_kp], last_blockhash);

        banks_client.process_transaction(tx).await.unwrap();

        let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

        assert_eq!(pool_state.pricing_program, TEST_PRICING_PROGRAM_SUCCESS);
    }

    // Basic rejection case
    {
        assert_ne!(TEST_PRICING_PROGRAM_FAILURE, DEFAULT_PRICING_PROGRAM);
        assert_ne!(TEST_PRICING_PROGRAM_FAILURE, TEST_PRICING_PROGRAM_SUCCESS);

        // A non-admin keypair
        let rando_kp = Keypair::new();

        let mut keys = SetPricingProgramFreeArgs {
            new_pricing_program: TEST_PRICING_PROGRAM_FAILURE,
            pool_state_acc: KeyedAccount {
                pubkey: POOL_STATE_ID,
                account: pool_state_account.clone(),
            },
        }
        .resolve()
        .unwrap();

        keys.admin = rando_kp.pubkey();

        let ix = set_pricing_program_ix(keys, SetPricingProgramIxArgs {}).unwrap();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
        tx.sign(&[&payer, &rando_kp], last_blockhash);

        let err = banks_client.process_transaction(tx).await.unwrap_err();

        let pool_state_acc = banks_client_get_pool_state_acc(&mut banks_client).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

        assert_program_error(err, ProgramError::InvalidArgument);
        assert_ne!(pool_state.pricing_program, TEST_PRICING_PROGRAM_FAILURE);
    }
}
