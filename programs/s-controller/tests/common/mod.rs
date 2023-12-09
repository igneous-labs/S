#![allow(unused)]

use generic_pool_calculator_test_utils::{
    program_test_add_mock_calculator_state, ProgramTestAddMockCalculatorStateArgs,
};
use marinade_calculator_lib::MarinadeSolValCalc;
use marinade_keys::msol;
use s_controller_interface::PoolState;
use s_controller_lib::program::POOL_STATE_ID;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use spl_calculator_lib::SplSolValCalc;
use test_utils::{
    jitosol, AddAccount, MARINADE_PROG_LAST_UPDATED_SLOT, SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT,
};

mod disable_pool_authority_list;
mod lst_state;
mod state;

pub use disable_pool_authority_list::*;
pub use lst_state::*;
pub use state::*;

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

#[derive(Clone, Copy, Default, Debug)]
pub struct JitoMarinadeProgramTestArgs {
    pub jitosol_sol_value: u64,
    pub msol_sol_value: u64,
    pub jitosol_reserves: u64,
    pub msol_reserves: u64,
    pub jitosol_protocol_fee_accumulator: u64,
    pub msol_protocol_fee_accumulator: u64,
    pub lp_token_mint: Pubkey,
    pub lp_token_supply: u64,
}

impl JitoMarinadeProgramTestArgs {
    pub fn with_lp_token_mint(mut self, lp_token_mint: Pubkey) -> Self {
        self.lp_token_mint = lp_token_mint;
        self
    }
}

/// Need to set pricing_program_id on returned PoolState
/// before adding
fn jito_marinade_base_program_test(
    JitoMarinadeProgramTestArgs {
        jitosol_sol_value,
        msol_sol_value,
        jitosol_reserves,
        msol_reserves,
        jitosol_protocol_fee_accumulator,
        msol_protocol_fee_accumulator,
        lp_token_mint,
        lp_token_supply,
    }: JitoMarinadeProgramTestArgs,
) -> (ProgramTest, PoolState) {
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
                protocol_fee_accumulator_amt: jitosol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: spl_calculator_lib::program::ID,
            },
            MockLstStateArgs {
                mint: msol::ID,
                sol_value: msol_sol_value,
                reserves_amt: msol_reserves,
                protocol_fee_accumulator_amt: msol_protocol_fee_accumulator,
                token_program: spl_token::ID,
                sol_value_calculator: marinade_calculator_lib::program::ID,
            },
        ],
    );
    program_test.add_account(lp_token_mint, mock_lp_mint(lp_token_mint, lp_token_supply));

    let total_sol_value = jitosol_sol_value + msol_sol_value;

    let mut pool_state = DEFAULT_POOL_STATE;
    pool_state.total_sol_value = total_sol_value;
    pool_state.lp_token_mint = lp_token_mint;

    (program_test, pool_state)
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
pub fn jito_marinade_no_fee_program_test(args: JitoMarinadeProgramTestArgs) -> ProgramTest {
    let (mut program_test, mut pool_state) = jito_marinade_base_program_test(args);
    program_test.add_program(
        "no_fee_pricing_program",
        no_fee_pricing_program::ID,
        processor!(no_fee_pricing_program::process_instruction),
    );
    pool_state.pricing_program = no_fee_pricing_program::ID;
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        pool_state_to_account(pool_state),
    );
    program_test
}

/// Creates a program test with only the PoolState account.
/// Useful for tests that only involve the PoolState account like certain admin functions
pub fn naked_pool_state_program_test(pool_state: PoolState) -> ProgramTest {
    let pool_state_account = pool_state_to_account(pool_state);
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );
    program_test.add_account(POOL_STATE_ID, pool_state_account);
    program_test
}
