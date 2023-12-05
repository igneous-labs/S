#![allow(dead_code)] // DELETEME
use bytemuck::{try_from_bytes, try_from_bytes_mut};

use flat_fee_interface::{FeeAccount, FlatFeeError, ProgramState};

pub fn try_program_state(program_state_acc_data: &[u8]) -> Result<&ProgramState, FlatFeeError> {
    try_from_bytes(program_state_acc_data).map_err(|_e| FlatFeeError::InvalidProgramStateData)
}

pub fn try_program_state_mut(
    program_state_acc_data: &mut [u8],
) -> Result<&mut ProgramState, FlatFeeError> {
    try_from_bytes_mut(program_state_acc_data).map_err(|_e| FlatFeeError::InvalidProgramStateData)
}

pub fn try_fee_account(fee_acc_data: &[u8]) -> Result<&FeeAccount, FlatFeeError> {
    try_from_bytes(fee_acc_data).map_err(|_e| FlatFeeError::UnsupportedLstMint) // TODO: should this be InvalidFeeAccountData?
}

pub fn try_fee_account_mut(fee_acc_data: &mut [u8]) -> Result<&mut FeeAccount, FlatFeeError> {
    try_from_bytes_mut(fee_acc_data).map_err(|_e| FlatFeeError::UnsupportedLstMint)
    // TODO: should this be InvalidFeeAccountData?
}
