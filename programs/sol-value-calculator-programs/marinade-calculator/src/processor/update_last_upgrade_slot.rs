use generic_pool_calculator_onchain::processor::{
    process_update_last_upgrade_slot_unchecked, verify_update_last_upgrade_slot,
};
use marinade_calculator_lib::MarinadeSolValCalc;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn process_update_last_upgrade_slot(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify_update_last_upgrade_slot::<MarinadeSolValCalc>(accounts)?;
    process_update_last_upgrade_slot_unchecked(checked)
}
