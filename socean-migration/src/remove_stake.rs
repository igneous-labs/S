//! Transfers ownership of a SPL stake pool stake account away.
//! Used for VSAs that have been DOS'd by SOL donations and removing the pool reserves

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_remove_stake(_accounts: &[AccountInfo]) -> ProgramResult {
    todo!()
}
