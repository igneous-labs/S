use s_controller_interface::PoolState;
use s_controller_lib::program::POOL_STATE_ID;
use solana_program_test::{processor, ProgramTest};

use crate::common::pool_state_to_account;

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
