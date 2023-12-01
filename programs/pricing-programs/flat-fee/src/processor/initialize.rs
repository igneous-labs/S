use flat_fee_lib::{
    initial_constants,
    processor::{process_initialize_unchecked, verify_initialize},
};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_initialize(accounts: &[AccountInfo]) -> ProgramResult {
    let checked = verify_initialize(accounts)?;
    process_initialize_unchecked(
        checked,
        initial_constants::initial_manager::ID,
        initial_constants::INITIAL_LP_WITHDRAWAL_FEE_BPS,
    )
}
