use borsh::BorshSerialize;
use sanctum_token_ratio::U64ValueRange;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{program::set_return_data, program_error::ProgramError};

const U64_VALUE_RANGE_SER_SIZE: usize = std::mem::size_of::<U64ValueRange>();

/// set_return_data() to result of calc_lst_to_sol()
pub fn process_lst_to_sol_unchecked<S: SolValueCalculator>(
    calc: &S,
    lst_amount: u64,
) -> Result<(), ProgramError> {
    let range = calc.calc_lst_to_sol(lst_amount)?;
    set_u64_value_range_return_data(range)
}

/// set_return_data() to result of calc_sol_to_lst()
pub fn process_sol_to_lst_unchecked<S: SolValueCalculator>(
    calc: &S,
    lamports_amount: u64,
) -> Result<(), ProgramError> {
    let range = calc.calc_sol_to_lst(lamports_amount)?;
    set_u64_value_range_return_data(range)
}

fn set_u64_value_range_return_data(range: U64ValueRange) -> Result<(), ProgramError> {
    let mut buf = [0u8; U64_VALUE_RANGE_SER_SIZE];
    range.serialize(&mut buf.as_mut())?;
    set_return_data(&buf);
    Ok(())
}
