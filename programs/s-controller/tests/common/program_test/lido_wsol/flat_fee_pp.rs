use flat_fee_test_utils::{MockFeeAccount, MockFeeAccountArgs, MockProgramState};
use sanctum_solana_test_utils::IntoAccount;
use solana_program_test::{processor, ProgramTest};

use crate::common::{MockPoolState, MockProtocolFeeBps};

use super::{lido_wsol_base_program_test, LidoWsolProgramTestArgs};

/// dont forget to
///
/// ```rust ignore
/// let ctx = program_test.start_with_context();
/// ctx.set_sysvar(&Clock {
///     epoch: LIDO_STATE_LAST_UPDATE_EPOCH,
///     ..Default::default()
/// });
/// ```
pub fn lido_wsol_flat_fee_program_test(
    args: LidoWsolProgramTestArgs,
    flat_fee_state: flat_fee_interface::ProgramState,
    mock_fee_accounts: [MockFeeAccountArgs; 2],
    MockProtocolFeeBps { trading, lp }: MockProtocolFeeBps,
) -> ProgramTest {
    let (mut program_test, mut pool_state) = lido_wsol_base_program_test(args);
    program_test.add_program(
        "flat_fee",
        flat_fee_interface::ID,
        processor!(flat_fee::entrypoint::process_instruction),
    );
    pool_state.pricing_program = flat_fee_interface::ID;
    pool_state.trading_protocol_fee_bps = trading;
    pool_state.lp_protocol_fee_bps = lp;
    program_test.add_account(
        flat_fee_lib::program::STATE_ID,
        MockProgramState(flat_fee_state).into_account(),
    );
    for mfa in mock_fee_accounts {
        let (acc, addr) = mfa.to_fee_account_and_addr(flat_fee_interface::ID);
        program_test.add_account(addr, MockFeeAccount(acc).into_account());
    }
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        MockPoolState(pool_state).into_account(),
    );
    program_test
}
