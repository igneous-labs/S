use sanctum_solana_test_utils::IntoAccount;
use solana_program_test::{processor, ProgramTest};

use crate::MockPoolState;

use super::{jito_marinade_base_program_test, JitoMarinadeProgramTestArgs};

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
        MockPoolState(pool_state).into_account(),
    );
    program_test
}
