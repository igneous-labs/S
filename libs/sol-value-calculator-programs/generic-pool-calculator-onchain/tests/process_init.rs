use generic_pool_calculator_interface::{init_ix, InitIxArgs};
use generic_pool_calculator_lib::{
    account_resolvers::InitRootAccounts, utils::try_calculator_state,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{signer::Signer, transaction::Transaction};

use crate::mock_calculator_program::MockCalculatorProgram;

mod mock_calculator_program {
    use generic_pool_calculator_interface::{InitAccounts, INIT_IX_ACCOUNTS_LEN};
    use generic_pool_calculator_lib::GenericPoolSolValCalc;
    use generic_pool_calculator_onchain::processor::process_init_unchecked;
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
        let accounts_arr: &[AccountInfo; INIT_IX_ACCOUNTS_LEN] = accounts.try_into().unwrap();
        let ix_accounts: InitAccounts = accounts_arr.into();
        process_init_unchecked::<MockCalculatorProgram>(ix_accounts, INITIAL_MANAGER_ID)
    }
}

#[tokio::test]
async fn init_basic() {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "mock_calculator_program",
        mock_calculator_program::ID,
        processor!(mock_calculator_program::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let root_accounts = InitRootAccounts {
        payer: payer.pubkey(),
    };
    let mut ix = init_ix(
        root_accounts.resolve::<MockCalculatorProgram>(),
        InitIxArgs {},
    )
    .unwrap();
    ix.program_id = mock_calculator_program::ID;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], recent_blockhash);

    assert!(banks_client.process_transaction(tx).await.is_ok());
    let state_account = banks_client
        .get_account(mock_calculator_program::STATE_ID)
        .await
        .unwrap()
        .unwrap();

    let state_bytes = state_account.data;
    let calc_state = try_calculator_state(&state_bytes).unwrap();

    assert_eq!(calc_state.last_upgrade_slot, 0);
    assert_eq!(
        calc_state.manager,
        mock_calculator_program::INITIAL_MANAGER_ID
    );
}

// TODO: test
// - calling twice
