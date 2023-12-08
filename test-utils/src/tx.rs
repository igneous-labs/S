use std::fmt::Display;

use num_traits::ToPrimitive;
use solana_program::{
    instruction::{Instruction, InstructionError},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program_test::{BanksClient, BanksClientError, BanksTransactionResultWithMetadata};
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    transaction::{Transaction, TransactionError},
    transaction_context::TransactionReturnData,
};

/// Extremely fucked up: TransactionReturnData truncates all rightmost zero bytes:
/// https://solana.stackexchange.com/questions/7141/program-return-data-to-client-error
pub fn zero_padded_return_data<const N: usize>(return_data: &[u8]) -> [u8; N] {
    let mut res = [0u8; N];
    let subslice = res.get_mut(..return_data.len()).unwrap();
    subslice.copy_from_slice(return_data);
    res
}

fn extract_ix_err(banks_client_err: BanksClientError) -> InstructionError {
    let tx_err = match banks_client_err {
        BanksClientError::TransactionError(e) => e,
        BanksClientError::SimulationError { err, .. } => err,
        _ => panic!("Unexpected BanksClientError {banks_client_err}"),
    };
    match tx_err {
        TransactionError::InstructionError(_, e) => e,
        _ => panic!("Unexpected TransactionError {tx_err}"),
    }
}

fn extract_ix_err_code(ix_err: &InstructionError) -> u32 {
    match ix_err {
        InstructionError::Custom(c) => *c,
        _ => panic!("Unexpected InstructionError {ix_err}"),
    }
}

pub fn assert_custom_err<E: Into<ProgramError> + Display + Copy>(
    banks_client_err: BanksClientError,
    expected_err: E,
) {
    let ix_err = extract_ix_err(banks_client_err);
    let actual_code = extract_ix_err_code(&ix_err);
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

/// Some types like SystemError implement different traits
pub fn assert_built_in_prog_err<E: ToPrimitive + Display>(
    banks_client_err: BanksClientError,
    expected_err: E,
) {
    let ix_err = extract_ix_err(banks_client_err);
    let actual_code = extract_ix_err_code(&ix_err);
    let expected_code = expected_err.to_u32().unwrap();
    assert_eq!(
        actual_code, expected_code,
        "Expected: {expected_err}. Actual: {ix_err}"
    );
}

pub fn assert_program_error(banks_client_err: BanksClientError, expected_err: ProgramError) {
    let ix_err = extract_ix_err(banks_client_err);
    let actual_err: ProgramError = ix_err.try_into().unwrap();
    assert_eq!(
        actual_err, expected_err,
        "Expected: {expected_err}. Actual: {actual_err}"
    );
}

/// Returns (program_that_set_return_data, u64_le_return_data)
pub async fn exec_get_u64_le_return_data(
    banks_client: &mut BanksClient,
    tx: Transaction,
) -> (Pubkey, u64) {
    let BanksTransactionResultWithMetadata { result, metadata } = banks_client
        .process_transaction_with_metadata(tx)
        .await
        .unwrap();

    result.unwrap(); // check result ok
    let TransactionReturnData { program_id, data } = metadata.unwrap().return_data.unwrap();
    let ret = u64::from_le_bytes(zero_padded_return_data(&data));
    (program_id, ret)
}

pub async fn exec_verify_u64_le_return_data(
    banks_client: &mut BanksClient,
    payer: &Keypair,
    last_blockhash: solana_program::hash::Hash,
    ix: Instruction,
    expected_return: u64,
) {
    let ix_program_id = ix.program_id;
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[payer], last_blockhash);
    let (program_id, ret) = exec_get_u64_le_return_data(banks_client, tx).await;
    assert_eq!(program_id, ix_program_id);
    assert_eq!(ret, expected_return);
}
