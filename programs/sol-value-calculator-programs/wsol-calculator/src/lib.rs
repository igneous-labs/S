use sanctum_onchain_utils::utils::{load_accounts, log_and_return_wrong_acc_err};
use sol_value_calculator_onchain::{process_lst_to_sol_unchecked, process_sol_to_lst_unchecked};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use wsol_calculator_interface::{
    lst_to_sol_verify_account_keys, LstToSolAccounts, LstToSolIxArgs, SolToLstIxArgs,
    WsolCalculatorProgramIx,
};
use wsol_calculator_lib::{WsolSolCalc, LST_TO_SOL_KEYS};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != wsol_calculator_lib::program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Assumes account interfaces of the 2 instructions are the same
    let accounts: LstToSolAccounts = load_accounts(accounts)?;
    lst_to_sol_verify_account_keys(&accounts, &LST_TO_SOL_KEYS)
        .map_err(log_and_return_wrong_acc_err)?;
    // accounts should all be read-only, no need to verify_account_privileges

    let ix = WsolCalculatorProgramIx::deserialize(&mut &instruction_data[..])?;
    solana_program::msg!("{:?}", ix);

    match ix {
        WsolCalculatorProgramIx::LstToSol(LstToSolIxArgs { amount }) => {
            process_lst_to_sol_unchecked(&WsolSolCalc, amount)
        }
        WsolCalculatorProgramIx::SolToLst(SolToLstIxArgs { amount }) => {
            process_sol_to_lst_unchecked(&WsolSolCalc, amount)
        }
    }
}
