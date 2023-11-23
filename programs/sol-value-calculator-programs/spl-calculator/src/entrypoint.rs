#![cfg(not(feature = "no-entrypoint"))]

use generic_pool_calculator_interface::GenericPoolCalculatorProgramIx;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != spl_calculator_lib::program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    GenericPoolCalculatorProgramIx::deserialize(&mut &instruction_data[..])?;
    todo!()
}
