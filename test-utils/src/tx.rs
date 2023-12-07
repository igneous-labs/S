use std::fmt::Display;

use solana_program::{instruction::InstructionError, program_error::ProgramError};
use solana_program_test::BanksClientError;
use solana_sdk::transaction::TransactionError;

/// Extremely fucked up TransactionReturnData truncates all rightmost zero bytes:
/// https://solana.stackexchange.com/questions/7141/program-return-data-to-client-error
pub fn zero_padded_return_data<const N: usize>(return_data: &[u8]) -> [u8; N] {
    let mut res = [0u8; N];
    let subslice = res.get_mut(..return_data.len()).unwrap();
    subslice.copy_from_slice(return_data);
    res
}

pub fn assert_is_custom_err<E: Into<ProgramError> + Display + Copy>(
    banks_client_err: BanksClientError,
    expected_err: E,
) {
    let tx_err = match banks_client_err {
        BanksClientError::TransactionError(e) => e,
        BanksClientError::SimulationError { err, .. } => err,
        _ => panic!("Unexpected BanksClientError {banks_client_err}"),
    };
    let ix_err = match tx_err {
        TransactionError::InstructionError(_, e) => e,
        _ => panic!("Unexpected TransactionError {tx_err}"),
    };
    let actual_code = match ix_err {
        InstructionError::Custom(c) => c,
        _ => panic!("Unexpected InstructionError {ix_err}"),
    };
    let expected_program_err: ProgramError = expected_err.into();
    let expected_code = match expected_program_err {
        ProgramError::Custom(c) => c,
        _ => panic!("Unexpected ProgramError {expected_program_err}. This doesn't look like a custom error type.")
    };
    assert_eq!(
        actual_code, expected_code,
        "Expected: {expected_err}. Actual: {ix_err}"
    );
}
