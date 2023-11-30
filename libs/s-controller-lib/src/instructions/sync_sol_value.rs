use s_controller_interface::{sync_sol_value_ix, SyncSolValueIxArgs, SyncSolValueKeys};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::utils::ix_extend_with_sol_value_calculator_accounts;

pub fn sync_sol_value_ix_full<
    K: Into<SyncSolValueKeys>,
    A: Into<SyncSolValueIxArgs>,
    S: Into<[AccountMeta; N]>,
    const N: usize,
>(
    accounts: K,
    args: A,
    sol_value_calculator_keys: S,
    sol_value_calculator_program_id: Pubkey,
) -> std::io::Result<Instruction> {
    let mut ix = sync_sol_value_ix(accounts, args)?;
    ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        sol_value_calculator_keys,
        sol_value_calculator_program_id,
    );
    Ok(ix)
}
