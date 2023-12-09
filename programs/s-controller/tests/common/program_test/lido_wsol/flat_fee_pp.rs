use flat_fee_test_utils::{
    fee_account_to_account, flat_fee_program_state_to_account, MockFeeAccountArgs,
};
use solana_program_test::{processor, ProgramTest};

use crate::common::{pool_state_to_account, MockProtocolFeeBps};

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
        flat_fee_program_state_to_account(flat_fee_state),
    );
    for mfa in mock_fee_accounts {
        let (acc, addr) = mfa.to_fee_account_and_addr();
        program_test.add_account(addr, fee_account_to_account(acc));
    }
    program_test.add_account(
        s_controller_lib::program::POOL_STATE_ID,
        pool_state_to_account(pool_state),
    );
    program_test
}
