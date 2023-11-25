use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{program::set_return_data, program_error::ProgramError};

/// set_return_data() to result of calc_lst_to_sol()
pub fn process_lst_to_sol_unchecked<S: SolValueCalculator>(
    calc: &S,
    lst_amount: u64,
) -> Result<(), ProgramError> {
    let lamports_amount = calc.calc_lst_to_sol(lst_amount)?;
    let lamports_amount_le = lamports_amount.to_le_bytes();
    set_return_data(&lamports_amount_le);
    Ok(())
}

/// set_return_data() to result of calc_sol_to_lst()
pub fn process_sol_to_lst_unchecked<S: SolValueCalculator>(
    calc: &S,
    lamports_amount: u64,
) -> Result<(), ProgramError> {
    let lst_amount = calc.calc_sol_to_lst(lamports_amount)?;
    let lst_amount_le = lst_amount.to_le_bytes();
    set_return_data(&lst_amount_le);
    Ok(())
}
