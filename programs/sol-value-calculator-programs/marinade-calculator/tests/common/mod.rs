use generic_pool_calculator_lib::GenericPoolSolValCalc;
use generic_pool_calculator_test_utils::{
    GenericPoolCalculatorProgramTest, MockCalculatorStateAccountArgs,
};
use marinade_calculator_lib::MarinadeSolValCalc;
use sanctum_solana_test_utils::ExtendedProgramTest;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use test_utils::MARINADE_PROG_LAST_UPDATED_SLOT;

pub fn marinade_normal_program_test() -> ProgramTest {
    let mut program_test = ProgramTest::default();
    // name must match <name>.so filename
    program_test.add_program(
        "marinade_calculator",
        marinade_calculator_lib::program::ID,
        processor!(marinade_calculator::entrypoint::process_instruction),
    );
    program_test
        .add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager: Pubkey::default(),
            last_upgrade_slot: MARINADE_PROG_LAST_UPDATED_SLOT,
            owner: MarinadeSolValCalc::ID,
        })
        .add_test_fixtures_account("marinade-state.json")
        .add_test_fixtures_account("marinade-prog.json")
        .add_test_fixtures_account("marinade-prog-data.json")
}
