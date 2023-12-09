use generic_pool_calculator_test_utils::{
    program_test_add_mock_calculator_state, ProgramTestAddMockCalculatorStateArgs,
};
use marinade_calculator_lib::MarinadeSolValCalc;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use spl_calculator_lib::SplSolValCalc;
use test_utils::{
    mock_token_account, AddAccount, MockTokenAccountArgs, MARINADE_PROG_LAST_UPDATED_SLOT,
    SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
};

/// Adds the spl sol value calculator program, a mock calculator state
/// and the spl stake pool program to a ProgramTest
pub fn add_spl_progs(mut program_test: ProgramTest) -> ProgramTest {
    // name must match <name>.so filename
    program_test.add_program(
        "spl_calculator",
        spl_calculator_lib::program::ID,
        processor!(spl_calculator::entrypoint::process_instruction),
    );
    program_test_add_mock_calculator_state::<SplSolValCalc>(
        ProgramTestAddMockCalculatorStateArgs {
            program_test: &mut program_test,
            manager: Pubkey::default(),
            last_upgrade_slot: SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
        },
    );
    program_test
        .add_test_fixtures_account("spl-stake-pool-prog.json")
        .add_test_fixtures_account("spl-stake-pool-prog-data.json")
}

/// Adds the marinade sol value calculator program, a mock calculator state
/// and the marinade program to a ProgramTest
pub fn add_marinade_progs(mut program_test: ProgramTest) -> ProgramTest {
    // name must match <name>.so filename
    program_test.add_program(
        "marinade_calculator",
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
        .add_test_fixtures_account("marinade-prog.json")
        .add_test_fixtures_account("marinade-prog-data.json")
}

/// Add jito stake pool and jitoSOL mint to a ProgramTest
pub fn add_jito_stake_pool(program_test: ProgramTest) -> ProgramTest {
    program_test
        .add_test_fixtures_account("jito-stake-pool.json")
        .add_test_fixtures_account("jitosol-mint.json")
}

/// Add marinade state and mSOL mint to a ProgramTest
pub fn add_marinade_stake_pool(program_test: ProgramTest) -> ProgramTest {
    program_test
        .add_test_fixtures_account("marinade-state.json")
        .add_test_fixtures_account("msol-mint.json")
}

pub fn add_mock_token_account(
    program_test: &mut ProgramTest,
    token_account: MockTokenAccountArgs,
) -> Pubkey {
    let token_acc_addr = Pubkey::new_unique();
    program_test.add_account(token_acc_addr, mock_token_account(token_account));
    token_acc_addr
}
