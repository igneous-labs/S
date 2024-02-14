use generic_pool_calculator_onchain::processor::{
    process_update_last_upgrade_slot_unchecked, verify_update_last_upgrade_slot,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_lib::SanctumSplSolValCalc;

pub fn process_update_last_upgrade_slot(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify_update_last_upgrade_slot::<SanctumSplSolValCalc>(accounts)?;
    process_update_last_upgrade_slot_unchecked(checked)
}
