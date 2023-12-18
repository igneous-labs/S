use generic_pool_calculator_lib::GenericPoolSolValCalc;
use generic_pool_calculator_test_utils::{
    GenericPoolCalculatorProgramTest, MockCalculatorStateAccountArgs,
};
use lido_calculator_lib::LidoSolValCalc;
use sanctum_solana_test_utils::ExtendedProgramTest;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use test_utils::LIDO_PROG_LAST_UPDATED_SLOT;

pub fn lido_normal_program_test() -> ProgramTest {
    let mut program_test = ProgramTest::default();
    // name must match <name>.so filename
    program_test.add_program(
        "lido_calculator",
        lido_calculator_lib::program::ID,
        processor!(lido_calculator::entrypoint::process_instruction),
    );
    program_test
        .add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager: Pubkey::default(),
            last_upgrade_slot: LIDO_PROG_LAST_UPDATED_SLOT,
            owner: LidoSolValCalc::ID,
        })
        .add_test_fixtures_account("lido-state.json")
        .add_test_fixtures_account("lido-prog.json")
        .add_test_fixtures_account("lido-prog-data.json")
}
