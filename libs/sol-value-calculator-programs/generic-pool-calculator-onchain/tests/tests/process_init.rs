use generic_pool_calculator_interface::{init_ix, InitIxArgs};
use generic_pool_calculator_lib::{account_resolvers::InitFreeArgs, utils::try_calculator_state};
use solana_program::{
    hash::Hash,
    system_instruction::{self, SystemError},
};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use mock_calculator_program::MockCalculatorProgram;
use test_utils::{assert_built_in_prog_err, banks_client_get_account};

mod mock_calculator_program {
    use generic_pool_calculator_lib::GenericPoolSolValCalc;
    use generic_pool_calculator_onchain::processor::{process_init_unchecked, verify_init};
    use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

    sanctum_macros::declare_program_keys!(
        "8kbLzKfKo5gjbGQf2HmULGGTXQx6hnfYGJ8inL1zvVeL",
        [("state", b"state"), ("initial_manager", b"man"),]
    );

    pub struct MockCalculatorProgram;

    impl GenericPoolSolValCalc for MockCalculatorProgram {
        // unused
        const POOL_PROGRAM_ID: Pubkey = Pubkey::new_from_array([0; 32]);
        // unused
        const POOL_PROGRAM_PROGDATA_ID: Pubkey = Pubkey::new_from_array([0; 32]);
        const CALCULATOR_STATE_PDA: Pubkey = STATE_ID;
        const CALCULATOR_STATE_BUMP: u8 = STATE_BUMP;
        const ID: Pubkey = ID;
    }

    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let checked = verify_init::<MockCalculatorProgram>(accounts)?;
        process_init_unchecked::<MockCalculatorProgram>(checked, INITIAL_MANAGER_ID)
    }
}

fn mock_prog_program_test() -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "mock_calculator_program",
        mock_calculator_program::ID,
        processor!(mock_calculator_program::process_instruction),
    );
    program_test
}

async fn exec_init_success(banks_client: &mut BanksClient, payer: &Keypair, last_blockhash: Hash) {
    let free_args = InitFreeArgs {
        payer: payer.pubkey(),
    };
    let mut ix = init_ix(free_args.resolve::<MockCalculatorProgram>(), InitIxArgs {}).unwrap();
    ix.program_id = mock_calculator_program::ID;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[payer], last_blockhash);

    assert!(banks_client.process_transaction(tx).await.is_ok());
    let state_account =
        banks_client_get_account(banks_client, mock_calculator_program::STATE_ID).await;

    let state_bytes = state_account.data;
    let calc_state = try_calculator_state(&state_bytes).unwrap();

    assert_eq!(calc_state.last_upgrade_slot, 0);
    assert_eq!(
        calc_state.manager,
        mock_calculator_program::INITIAL_MANAGER_ID
    );
}

#[tokio::test]
async fn init_basic() {
    let program_test = mock_prog_program_test();
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;
    exec_init_success(&mut banks_client, &payer, last_blockhash).await;
}

#[tokio::test]
async fn fail_init_second_time() {
    let program_test = mock_prog_program_test();
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;
    exec_init_success(&mut banks_client, &payer, last_blockhash).await;

    // 2nd time.
    let free_args = InitFreeArgs {
        payer: payer.pubkey(),
    };
    let mut ix = init_ix(free_args.resolve::<MockCalculatorProgram>(), InitIxArgs {}).unwrap();
    ix.program_id = mock_calculator_program::ID;
    // Must change the transaction else
    // runtime will treat it as a duplicate and return previous Ok(()) result
    let dummy_transfer_ix = system_instruction::transfer(&payer.pubkey(), &payer.pubkey(), 1);
    let mut tx = Transaction::new_with_payer(&[ix, dummy_transfer_ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);

    let err = banks_client.process_transaction(tx).await.unwrap_err();
    assert_built_in_prog_err(err, SystemError::AccountAlreadyInUse);
}
