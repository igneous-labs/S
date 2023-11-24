use bytemuck::{try_from_bytes, try_from_bytes_mut};
use generic_pool_calculator_interface::{CalculatorState, GenericPoolCalculatorError};
use solana_program::{bpf_loader_upgradeable::UpgradeableLoaderState, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;

/// Attempts to deserialize a program account and read the
/// programdata_address contained within
pub fn read_programdata_addr<D: ReadonlyAccountData>(
    prog_acc: &D,
) -> Result<Pubkey, GenericPoolCalculatorError> {
    let prog_bytes = &prog_acc.data();
    let prog: UpgradeableLoaderState = bincode::deserialize(prog_bytes)
        .map_err(|_e| GenericPoolCalculatorError::InvalidStakePoolProgramData)?;
    if let UpgradeableLoaderState::Program {
        programdata_address,
    } = prog
    {
        Ok(programdata_address)
    } else {
        Err(GenericPoolCalculatorError::InvalidStakePoolProgramData)
    }
}

/// Attempts to deserialize the header of the program data account
/// of a stake pool program program data account and return
/// (last_upgrade_slot, upgrade_authority)
pub fn read_stake_pool_progdata_meta<D: ReadonlyAccountData>(
    stake_pool_prog_data_acc: &D,
) -> Result<(u64, Option<Pubkey>), GenericPoolCalculatorError> {
    let data = stake_pool_prog_data_acc.data();
    let meta_slice = data
        .get(..UpgradeableLoaderState::size_of_programdata_metadata())
        .ok_or(GenericPoolCalculatorError::InvalidStakePoolProgramData)?;
    let meta: UpgradeableLoaderState = bincode::deserialize(meta_slice)
        .map_err(|_e| GenericPoolCalculatorError::InvalidStakePoolProgramData)?;
    if let UpgradeableLoaderState::ProgramData {
        slot,
        upgrade_authority_address,
    } = meta
    {
        Ok((slot, upgrade_authority_address))
    } else {
        Err(GenericPoolCalculatorError::InvalidStakePoolProgramData)
    }
}

/// Tries to reinterpret calculator_state_acc_data bytes as a CalculatorState
pub fn try_calculator_state(
    calculator_state_acc_data: &[u8],
) -> Result<&CalculatorState, GenericPoolCalculatorError> {
    try_from_bytes(calculator_state_acc_data)
        .map_err(|_e| GenericPoolCalculatorError::InvalidCalculatorStateData)
}

/// Tries to reinterpret calculator_state_acc_data bytes as a mutable CalculatorState
pub fn try_calculator_state_mut(
    calculator_state_acc_data: &mut [u8],
) -> Result<&mut CalculatorState, GenericPoolCalculatorError> {
    try_from_bytes_mut(calculator_state_acc_data)
        .map_err(|_e| GenericPoolCalculatorError::InvalidCalculatorStateData)
}
