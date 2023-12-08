use generic_pool_calculator_onchain::processor::{
    process_set_manager_unchecked, verify_set_manager,
};
use lido_calculator_lib::LidoSolValCalc;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn process_set_manager(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify_set_manager::<LidoSolValCalc>(accounts)?;
    process_set_manager_unchecked(checked)
}
