use s_controller_interface::StartRebalanceIxArgs;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_start_rebalance(
    _accounts: &[AccountInfo],
    _args: StartRebalanceIxArgs,
) -> ProgramResult {
    todo!()
}
