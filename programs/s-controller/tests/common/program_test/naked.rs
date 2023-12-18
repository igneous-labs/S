use s_controller_interface::PoolState;
use s_controller_lib::program::POOL_STATE_ID;
use sanctum_solana_test_utils::IntoAccount;
use solana_program_test::{processor, ProgramTest};

use crate::common::MockPoolState;

/// Creates a program test with only the PoolState account.
/// Useful for tests that only involve the PoolState account like certain admin functions
pub fn naked_pool_state_program_test(pool_state: PoolState) -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "s_controller",
        s_controller_lib::program::ID,
        processor!(s_controller::entrypoint::process_instruction),
    );
    program_test.add_account(POOL_STATE_ID, MockPoolState(pool_state).into_account());
    program_test
}
