use s_controller_interface::SyncSolValueIxArgs;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_sync_sol_value(
    _accounts: &[AccountInfo],
    _args: SyncSolValueIxArgs,
) -> ProgramResult {
    todo!()
}
