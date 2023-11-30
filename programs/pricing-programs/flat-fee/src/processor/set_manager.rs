use flat_fee_lib::processor::{process_set_manager_unchecked, verify_set_manager};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_set_manager(accounts: &[AccountInfo]) -> ProgramResult {
    let checked = verify_set_manager(accounts)?;
    process_set_manager_unchecked(checked)
}
