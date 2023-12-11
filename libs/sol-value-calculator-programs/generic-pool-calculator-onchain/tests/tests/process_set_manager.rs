use generic_pool_calculator_interface::{set_manager_ix, SetManagerIxArgs, SetManagerKeys};
use generic_pool_calculator_lib::{
    account_resolvers::SetManagerFreeArgs, utils::try_calculator_state,
};
use generic_pool_calculator_test_utils::{
    mock_calculator_state_account, MockCalculatorStateAccountArgs,
};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use mock_calculator_program::MockCalculatorProgram;
use test_utils::{assert_program_error, banks_client_get_account};

mod mock_calculator_program {
    use generic_pool_calculator_lib::GenericPoolSolValCalc;
    use generic_pool_calculator_onchain::processor::{
        process_set_manager_unchecked, verify_set_manager,
    };
    use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
    use spl_stake_pool_keys::{spl_stake_pool_program, spl_stake_pool_program_progdata};

    sanctum_macros::declare_program_keys!(
        "8kbLzKfKo5gjbGQf2HmULGGTXQx6hnfYGJ8inL1zvVeL",
        [("state", b"state")]
    );

    pub struct MockCalculatorProgram;

    impl GenericPoolSolValCalc for MockCalculatorProgram {
        const POOL_PROGRAM_ID: Pubkey = spl_stake_pool_program::ID;
        const POOL_PROGRAM_PROGDATA_ID: Pubkey = spl_stake_pool_program_progdata::ID;
        const CALCULATOR_STATE_PDA: Pubkey = STATE_ID;
        const CALCULATOR_STATE_BUMP: u8 = STATE_BUMP;
        const ID: Pubkey = ID;
    }

    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let checked = verify_set_manager::<MockCalculatorProgram>(accounts)?;
        process_set_manager_unchecked(checked)
    }
}

fn mock_prog_program_test(manager: Pubkey) -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "mock_calculator_program",
        mock_calculator_program::ID,
        processor!(mock_calculator_program::process_instruction),
    );
    let mock_state = mock_calculator_state_account(MockCalculatorStateAccountArgs {
        manager,
        last_upgrade_slot: Default::default(),
        owner: mock_calculator_program::ID,
    });
    program_test.add_account(mock_calculator_program::STATE_ID, mock_state);
    program_test
}

async fn verify_correct_manager(banks_client: &mut BanksClient, expected_manager: Pubkey) {
    let state_account =
        banks_client_get_account(banks_client, mock_calculator_program::STATE_ID).await;
    let state_bytes = state_account.data;
    let calc_state = try_calculator_state(&state_bytes).unwrap();
    assert_eq!(calc_state.manager, expected_manager);
}

#[tokio::test]
async fn set_manager_basic() {
    let manager = Keypair::new();
    let new_manager = Pubkey::new_unique();

    let program_test = mock_prog_program_test(manager.pubkey());
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mock_state =
        banks_client_get_account(&mut banks_client, mock_calculator_program::STATE_ID).await;
    verify_correct_manager(&mut banks_client, manager.pubkey()).await;

    let free_args = SetManagerFreeArgs {
        new_manager,
        state: KeyedAccount {
            pubkey: mock_calculator_program::STATE_ID,
            account: mock_state,
        },
    };
    let mut ix = set_manager_ix(
        free_args.resolve::<MockCalculatorProgram>().unwrap(),
        SetManagerIxArgs {},
    )
    .unwrap();
    ix.program_id = mock_calculator_program::ID;
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &manager], recent_blockhash);
    assert!(banks_client.process_transaction(tx).await.is_ok());

    verify_correct_manager(&mut banks_client, new_manager).await;
}

#[tokio::test]
async fn fail_set_manager_unauthorized_manager() {
    let manager = Pubkey::new_unique();

    let program_test = mock_prog_program_test(manager);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    verify_correct_manager(&mut banks_client, manager).await;
    let mut ix = set_manager_ix(
        SetManagerKeys {
            manager: payer.pubkey(),
            new_manager: payer.pubkey(),
            state: mock_calculator_program::STATE_ID,
        },
        SetManagerIxArgs {},
    )
    .unwrap();
    ix.program_id = mock_calculator_program::ID;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);
    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_program_error(err, ProgramError::InvalidArgument);
}

#[tokio::test]
async fn fail_set_manager_missing_signature() {
    let manager = Pubkey::new_unique();

    let program_test = mock_prog_program_test(manager);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    verify_correct_manager(&mut banks_client, manager).await;
    let mut ix = set_manager_ix(
        SetManagerKeys {
            manager,
            new_manager: payer.pubkey(),
            state: mock_calculator_program::STATE_ID,
        },
        SetManagerIxArgs {},
    )
    .unwrap();
    ix.accounts[0].is_signer = false;
    ix.program_id = mock_calculator_program::ID;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);
    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_program_error(err, ProgramError::MissingRequiredSignature);
}
