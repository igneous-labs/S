use generic_pool_calculator_test_utils::{
    program_test_add_mock_calculator_state, ProgramTestAddMockCalculatorStateArgs,
};
use lido_calculator_lib::LidoSolValCalc;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use test_utils::{AddAccount, LIDO_PROG_LAST_UPDATED_SLOT};

pub fn lido_normal_program_test() -> ProgramTest {
    let mut program_test = ProgramTest::default();
    // name must match <name>.so filename
    program_test.add_program(
        "lido_calculator",
        lido_calculator_lib::program::ID,
        processor!(lido_calculator::entrypoint::process_instruction),
    );

    program_test_add_mock_calculator_state::<LidoSolValCalc>(
        ProgramTestAddMockCalculatorStateArgs {
            program_test: &mut program_test,
            manager: Pubkey::default(),
            last_upgrade_slot: LIDO_PROG_LAST_UPDATED_SLOT,
        },
    );
    program_test
        .add_test_fixtures_account("lido-state.json")
        .add_test_fixtures_account("lido-prog.json")
        .add_test_fixtures_account("lido-prog-data.json")
}
