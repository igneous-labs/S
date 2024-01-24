use generic_pool_calculator_test_utils::{
    GenericPoolCalculatorProgramTest, MockCalculatorStateAccountArgs,
};
use solana_program::pubkey::Pubkey;

pub trait GpcSplProgramTest {
    fn add_mock_spl_calculator_state(self, last_upgrade_slot: u64, manager: Pubkey) -> Self;
}

impl<T: GenericPoolCalculatorProgramTest> GpcSplProgramTest for T {
    fn add_mock_spl_calculator_state(self, last_upgrade_slot: u64, manager: Pubkey) -> Self {
        self.add_mock_calculator_state(MockCalculatorStateAccountArgs {
            manager,
            last_upgrade_slot,
            owner: spl_calculator_lib::program::ID,
        })
    }
}
