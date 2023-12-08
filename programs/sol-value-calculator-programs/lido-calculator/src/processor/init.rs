use generic_pool_calculator_onchain::processor::{process_init_unchecked, verify_init};
use lido_calculator_lib::{initial_manager, LidoSolValCalc};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn process_init(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify_init::<LidoSolValCalc>(accounts)?;
    process_init_unchecked::<LidoSolValCalc>(checked, initial_manager::ID)
}
