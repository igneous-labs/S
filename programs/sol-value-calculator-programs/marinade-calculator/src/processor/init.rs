use generic_pool_calculator_onchain::processor::{process_init_unchecked, verify_init};
use marinade_calculator_lib::{initial_manager, MarinadeSolValCalc};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn process_init(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify_init::<MarinadeSolValCalc>(accounts)?;
    process_init_unchecked::<MarinadeSolValCalc>(checked, initial_manager::ID)
}
