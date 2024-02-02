use flat_fee_interface::ProgramState;
use flat_fee_lib::program;
use sanctum_solana_test_utils::{ExtendedProgramTest, IntoAccount};
use solana_program_test::ProgramTest;

use crate::MockProgramState;

pub trait FlatFeeProgramTest {
    fn add_mock_program_state(self, program_state: ProgramState) -> Self;
}

impl FlatFeeProgramTest for ProgramTest {
    fn add_mock_program_state(self, program_state: ProgramState) -> Self {
        self.add_account_chained(
            program::STATE_ID,
            MockProgramState(program_state).into_account(),
        )
    }
}
