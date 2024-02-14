use generic_pool_calculator_interface::GenericPoolCalculatorProgramIx;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_calculator_lib::sanctum_spl_sol_val_calc_program;

use crate::processor::{
    process_init, process_lst_to_sol, process_set_manager, process_sol_to_lst,
    process_update_last_upgrade_slot,
};

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != sanctum_spl_sol_val_calc_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let ix = GenericPoolCalculatorProgramIx::deserialize(instruction_data)?;
    solana_program::msg!("{:?}", ix);

    match ix {
        GenericPoolCalculatorProgramIx::LstToSol(args) => process_lst_to_sol(accounts, args),
        GenericPoolCalculatorProgramIx::SolToLst(args) => process_sol_to_lst(accounts, args),
        GenericPoolCalculatorProgramIx::UpdateLastUpgradeSlot => {
            process_update_last_upgrade_slot(accounts)
        }
        GenericPoolCalculatorProgramIx::SetManager => process_set_manager(accounts),
        GenericPoolCalculatorProgramIx::Init => process_init(accounts),
    }
}
