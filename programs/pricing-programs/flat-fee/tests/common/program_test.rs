use flat_fee_interface::ProgramState;
use flat_fee_lib::program::STATE_ID;
use solana_program_test::{processor, ProgramTest};

use super::{fee_account_to_account, program_state_to_account, MockFeeAccountArgs};

pub fn normal_program_test(
    state: ProgramState,
    mock_fee_accounts: &[MockFeeAccountArgs],
) -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "flat_fee",
        flat_fee_lib::program::ID,
        processor!(flat_fee::entrypoint::process_instruction),
    );
    program_test.add_account(STATE_ID, program_state_to_account(state));
    for mfa in mock_fee_accounts {
        let (acc, addr) = mfa.to_fee_account_and_addr();
        program_test.add_account(addr, fee_account_to_account(acc));
    }
    program_test
}
