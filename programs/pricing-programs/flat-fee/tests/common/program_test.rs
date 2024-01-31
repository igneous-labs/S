use flat_fee_interface::ProgramState;
use flat_fee_lib::program::STATE_ID;
use flat_fee_test_utils::{MockFeeAccount, MockFeeAccountArgs, MockProgramState};
use sanctum_solana_test_utils::IntoAccount;
use solana_program_test::{processor, ProgramTest};

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
    program_test.add_account(STATE_ID, MockProgramState(state).into_account());
    for mfa in mock_fee_accounts {
        let (acc, addr) = mfa.to_fee_account_and_addr(flat_fee_lib::program::ID);
        program_test.add_account(addr, MockFeeAccount(acc).into_account());
    }
    program_test
}
