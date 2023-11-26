use generic_pool_calculator_interface::GenericPoolCalculatorProgramIx;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::processor::{
    process_init, process_lst_to_sol, process_set_manager, process_sol_to_lst,
    process_update_last_upgrade_slot,
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != spl_calculator_lib::program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let ix = GenericPoolCalculatorProgramIx::deserialize(&mut &instruction_data[..])?;
    solana_program::msg!("{:?}", ix);

    match ix {
        GenericPoolCalculatorProgramIx::LstToSol(args) => process_lst_to_sol(accounts, args),
        GenericPoolCalculatorProgramIx::SolToLst(args) => process_sol_to_lst(accounts, args),
        GenericPoolCalculatorProgramIx::UpdateLastUpgradeSlot(_args) => {
            process_update_last_upgrade_slot(accounts)
        }
        GenericPoolCalculatorProgramIx::SetManager(_args) => process_set_manager(accounts),
        GenericPoolCalculatorProgramIx::Init(_args) => process_init(accounts),
    }
}
