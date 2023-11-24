#![cfg(not(feature = "no-entrypoint"))]

use generic_pool_calculator_interface::GenericPoolCalculatorProgramIx;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::processor::{process_init, process_set_manager};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != spl_calculator_lib::program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    match GenericPoolCalculatorProgramIx::deserialize(&mut &instruction_data[..])? {
        GenericPoolCalculatorProgramIx::Init(_args) => process_init(accounts),
        GenericPoolCalculatorProgramIx::SetManager(_args) => process_set_manager(accounts),
        _ => todo!(),
    }
}
