use generic_pool_calculator_interface::SolToLstIxArgs;
use marinade_calculator_lib::MarinadeStateCalc;
use sol_value_calculator_onchain::process_sol_to_lst_unchecked;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use super::lst_sol_common::verify_lst_sol_common;

pub fn process_sol_to_lst(
    accounts: &[AccountInfo],
    SolToLstIxArgs { amount }: SolToLstIxArgs,
) -> Result<(), ProgramError> {
    let marinade_state = verify_lst_sol_common(accounts)?;
    process_sol_to_lst_unchecked(&MarinadeStateCalc(marinade_state), amount)
}
