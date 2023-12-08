use generic_pool_calculator_onchain::processor::{
    process_update_last_upgrade_slot_unchecked, verify_update_last_upgrade_slot,
};
use lido_calculator_lib::LidoSolValCalc;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn process_update_last_upgrade_slot(accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let checked = verify_update_last_upgrade_slot::<LidoSolValCalc>(accounts)?;
    process_update_last_upgrade_slot_unchecked(checked)
}
