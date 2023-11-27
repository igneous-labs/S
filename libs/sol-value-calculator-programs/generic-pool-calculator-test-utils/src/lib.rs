use generic_pool_calculator_lib::{
    utils::try_calculator_state_mut, GenericPoolSolValCalc, CALCULATOR_STATE_SIZE,
};
use solana_program_test::ProgramTest;
use solana_sdk::{account::Account, pubkey::Pubkey};

pub struct MockCalculatorStateAccountArgs {
    pub manager: Pubkey,
    pub last_upgrade_slot: u64,

    /// GenericPoolCalculator program ID
    pub owner: Pubkey,
}

pub fn mock_calculator_state_account(
    MockCalculatorStateAccountArgs {
        manager,
        last_upgrade_slot,
        owner,
    }: MockCalculatorStateAccountArgs,
) -> Account {
    let mut data = vec![0u8; CALCULATOR_STATE_SIZE];
    let state = try_calculator_state_mut(&mut data).unwrap();
    state.manager = manager;
    state.last_upgrade_slot = last_upgrade_slot;
    Account {
        lamports: 1_000_000_000, // just do 1 SOL lol
        data,
        owner,
        executable: false,
        rent_epoch: u64::MAX,
    }
}

pub struct ProgramTestAddMockCalculatorStateArgs<'me> {
    pub program_test: &'me mut ProgramTest,
    pub manager: Pubkey,
    pub last_upgrade_slot: u64,
}

pub fn program_test_add_mock_calculator_state<P: GenericPoolSolValCalc>(
    ProgramTestAddMockCalculatorStateArgs {
        program_test,
        manager,
        last_upgrade_slot,
    }: ProgramTestAddMockCalculatorStateArgs,
) {
    let mock_calculator_state = mock_calculator_state_account(MockCalculatorStateAccountArgs {
        manager,
        last_upgrade_slot,
        owner: P::ID,
    });
    program_test.add_account(P::CALCULATOR_STATE_PDA, mock_calculator_state);
}
