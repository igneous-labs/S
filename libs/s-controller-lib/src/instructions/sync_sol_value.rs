use s_controller_interface::{sync_sol_value_ix, SyncSolValueIxArgs, SyncSolValueKeys};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::utils::{ix_extend_with_sol_value_calculator_accounts, try_from_int_err_to_io_err};

pub fn sync_sol_value_ix_full<K: Into<SyncSolValueKeys>, A: Into<SyncSolValueIxArgs>>(
    accounts: K,
    args: A,
    sol_value_calculator_accounts: &[AccountMeta],
    sol_value_calculator_program_id: Pubkey,
) -> std::io::Result<Instruction> {
    let mut ix = sync_sol_value_ix(accounts, args)?;
    ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        sol_value_calculator_accounts,
        sol_value_calculator_program_id,
    )
    .map_err(try_from_int_err_to_io_err)?;
    Ok(ix)
}
