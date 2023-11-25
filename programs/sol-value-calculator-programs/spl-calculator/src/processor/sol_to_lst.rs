use generic_pool_calculator_interface::SolToLstIxArgs;
use sol_value_calculator_onchain::process_sol_to_lst_unchecked;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_lib::SplStakePoolCalc;

use super::lst_sol_common::verify_lst_sol_common;

pub fn process_sol_to_lst(
    accounts: &[AccountInfo],
    SolToLstIxArgs { amount }: SolToLstIxArgs,
) -> Result<(), ProgramError> {
    let stake_pool = verify_lst_sol_common(accounts)?;
    process_sol_to_lst_unchecked(&SplStakePoolCalc(stake_pool), amount)
}
