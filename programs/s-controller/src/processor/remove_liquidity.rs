use s_controller_interface::RemoveLiquidityIxArgs;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

pub fn process_remove_liquidity(
    _accounts: &[AccountInfo],
    _args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    todo!()
}
