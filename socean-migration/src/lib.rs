mod keys;
mod migrate;
mod remove_stake;

use migrate::process_migrate;
use remove_stake::process_remove_stake;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

solana_program::entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let discm = data.first().ok_or(ProgramError::InvalidInstructionData)?;
    match discm {
        0 => process_migrate(accounts),
        1 => process_remove_stake(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
