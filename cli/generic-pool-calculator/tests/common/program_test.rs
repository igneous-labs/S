use generic_pool_calculator_test_utils::{
    GenericPoolCalculatorProgramTest, MockCalculatorStateAccountArgs,
};
use sanctum_solana_test_utils::ExtendedProgramTest;
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};

pub trait GpcSplProgramTest {
    fn add_spl_programs(self) -> Self;
    fn add_mock_spl_calculator_state(self, last_upgrade_slot: u64, manager: Pubkey) -> Self;
}

impl GpcSplProgramTest for ProgramTest {
    fn add_spl_programs(self) -> Self {
        // test against spl calculator
        let mut pt = self
            .add_test_fixtures_account("spl-stake-pool-prog.json")
            .add_test_fixtures_account("spl-stake-pool-prog-data.json");
        pt.add_program(
            "spl_calculator",
            spl_calculator_lib::program::ID,
            processor!(spl_calculator::entrypoint::process_instruction),
        );
        pt
    }

    fn add_mock_spl_calculator_state(self, last_upgrade_slot: u64, manager: Pubkey) -> Self {
        self.add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager,
            last_upgrade_slot,
            owner: spl_calculator_lib::program::ID,
        })
    }
}
