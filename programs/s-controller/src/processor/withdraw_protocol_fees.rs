use s_controller_interface::WithdrawProtocolFeesIxArgs;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_withdraw_protocol_fees(
    _accounts: &[AccountInfo],
    _args: WithdrawProtocolFeesIxArgs,
) -> ProgramResult {
    todo!()
}
