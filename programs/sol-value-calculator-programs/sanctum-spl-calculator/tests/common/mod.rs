use generic_pool_calculator_lib::GenericPoolSolValCalc;
use generic_pool_calculator_test_utils::{
    GenericPoolCalculatorProgramTest, MockCalculatorStateAccountArgs,
};
use sanctum_solana_test_utils::{ExtendedProgramTest, KeyedUiAccount};
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use solana_readonly_account::sdk::KeyedAccount;
use spl_calculator_lib::{sanctum_spl_sol_val_calc_program, SanctumSplSolValCalc};
use test_utils::SANCTUM_SPL_STAKE_POOL_PROG_LAST_UDPATED_SLOT;

pub struct PwrNormalProgramTest {
    pub program_test: ProgramTest,
    pub pwr_stake_pool: KeyedAccount,
    pub spl_stake_pool_prog: KeyedAccount,
}

pub fn pwr_normal_program_test() -> PwrNormalProgramTest {
    let mut program_test = ProgramTest::default();
    // name must match <name>.so filename
    program_test.add_program(
        "sanctum_spl_calculator",
        sanctum_spl_sol_val_calc_program::ID,
        processor!(sanctum_spl_calculator::entrypoint::process_instruction),
    );

    let sanctum_spl_stake_pool_prog_ui_acc =
        KeyedUiAccount::from_test_fixtures_file("sanctum-spl-prog.json");
    let pwr_stake_pool_ui_acc = KeyedUiAccount::from_test_fixtures_file("pwr-pool.json");

    let spl_stake_pool_prog = sanctum_spl_stake_pool_prog_ui_acc.to_keyed_account();
    let pwr_stake_pool = pwr_stake_pool_ui_acc.to_keyed_account();

    program_test = program_test
        .add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager: Pubkey::default(),
            last_upgrade_slot: SANCTUM_SPL_STAKE_POOL_PROG_LAST_UDPATED_SLOT,
            owner: SanctumSplSolValCalc::ID,
        })
        .add_keyed_ui_account(sanctum_spl_stake_pool_prog_ui_acc)
        .add_keyed_ui_account(pwr_stake_pool_ui_acc)
        .add_test_fixtures_account("sanctum-spl-prog-data.json");

    PwrNormalProgramTest {
        program_test,
        spl_stake_pool_prog,
        pwr_stake_pool,
    }
}
