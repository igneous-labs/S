use generic_pool_calculator_interface::LstToSolIxArgs;
use sol_value_calculator_onchain::process_lst_to_sol_unchecked;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_lib::SplStakePoolCalc;

use super::lst_sol_common::verify_lst_sol_common;

pub fn process_lst_to_sol(
    accounts: &[AccountInfo],
    LstToSolIxArgs { amount }: LstToSolIxArgs,
) -> Result<(), ProgramError> {
    let stake_pool = verify_lst_sol_common(accounts)?;
    process_lst_to_sol_unchecked(&SplStakePoolCalc(stake_pool), amount)
}
