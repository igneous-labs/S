use generic_pool_calculator_test_utils::{
    program_test_add_mock_calculator_state, ProgramTestAddMockCalculatorStateArgs,
};
use marinade_calculator_lib::MarinadeSolValCalc;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use test_utils::{AddAccount, MARINADE_PROG_LAST_UPDATED_SLOT};

pub fn marinade_normal_program_test() -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "marinade_sol_value_calculator",
        marinade_calculator_lib::program::ID,
        processor!(marinade_calculator::entrypoint::process_instruction),
    );

    program_test_add_mock_calculator_state::<MarinadeSolValCalc>(
        ProgramTestAddMockCalculatorStateArgs {
            program_test: &mut program_test,
            manager: Pubkey::default(),
            last_upgrade_slot: MARINADE_PROG_LAST_UPDATED_SLOT,
        },
    );
    program_test
        .add_test_fixtures_account("marinade-state.json")
        .add_test_fixtures_account("marinade-prog.json")
        .add_test_fixtures_account("marinade-prog-data.json")
}
