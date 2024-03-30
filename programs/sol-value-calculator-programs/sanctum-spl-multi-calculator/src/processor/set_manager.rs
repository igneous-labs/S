use generic_pool_calculator_onchain::processor::{
    process_set_manager_unchecked, verify_set_manager,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_lib::SanctumSplMultiSolValCalc;

pub fn process_set_manager(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify_set_manager::<SanctumSplMultiSolValCalc>(accounts)?;
    process_set_manager_unchecked(checked)
}
