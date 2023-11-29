use s_controller_interface::SwapExactOutIxArgs;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_swap_exact_out(
    _accounts: &[AccountInfo],
    _args: SwapExactOutIxArgs,
) -> ProgramResult {
    todo!()
}
