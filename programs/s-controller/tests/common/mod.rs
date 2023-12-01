use generic_pool_calculator_test_utils::{
    program_test_add_mock_calculator_state, ProgramTestAddMockCalculatorStateArgs,
};
use marinade_calculator_lib::MarinadeSolValCalc;
use marinade_keys::msol;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use spl_calculator_lib::SplSolValCalc;
use test_utils::{
    jitosol, AddAccount, MARINADE_PROG_LAST_UPDATED_SLOT, SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
};

mod lst_state;
mod state;
mod utils;

pub use lst_state::*;
pub use state::*;
pub use utils::*;

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

pub struct JitoMarinadeProgramTestArgs {
    pub jitosol_sol_value: u64,
    pub msol_sol_value: u64,
    pub jitosol_reserves: u64,
    pub msol_reserves: u64,
}

/// dont forget to
///
/// ```rust ignore
/// let ctx = program_test.start_with_context();
/// ctx.set_sysvar(&Clock {
///     epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
///     ..Default::default()
/// });
/// ```
pub fn jito_marinade_program_test(
    JitoMarinadeProgramTestArgs {
        jitosol_sol_value,
        msol_sol_value,
        jitosol_reserves,
        msol_reserves,
    }: JitoMarinadeProgramTestArgs,
) -> ProgramTest {
    let mut program_test = ProgramTest::default();

    // name must match <name>.so filename
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );
    program_test = add_spl_progs(program_test);
    program_test = add_marinade_progs(program_test);
    program_test = add_jito_stake_pool(program_test);
    program_test = add_marinade_stake_pool(program_test);
    program_test = program_test_add_mock_lst_states(
        program_test,
        &[
            MockLstStateArgs {
                mint: jitosol::ID,
                sol_value: jitosol_sol_value,
                reserves_amt: jitosol_reserves,
                sol_value_calculator: spl_calculator_lib::program::ID,
            },
            MockLstStateArgs {
                mint: msol::ID,
                sol_value: msol_sol_value,
                reserves_amt: msol_reserves,
                sol_value_calculator: marinade_calculator_lib::program::ID,
            },
        ],
    );
    let total_sol_value = jitosol_sol_value + msol_sol_value;

    let mut pool_state = DEFAULT_POOL_STATE;
    // TODO: set other state vars
    pool_state.total_sol_value = total_sol_value;

    program_test.add_account(
        s_controller_lib::program::STATE_ID,
        pool_state_to_account(pool_state),
    );

    program_test
}
