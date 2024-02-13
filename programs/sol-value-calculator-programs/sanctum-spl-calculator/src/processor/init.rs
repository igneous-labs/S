use generic_pool_calculator_onchain::processor::{process_init_unchecked, verify_init};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_lib::{initial_manager, SanctumSplSolValCalc};

pub fn process_init(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify_init::<SanctumSplSolValCalc>(accounts)?;
    process_init_unchecked::<SanctumSplSolValCalc>(checked, initial_manager::ID)
}
