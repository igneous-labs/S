use s_controller_interface::{
    sync_sol_value_ix, SControllerError, SyncSolValueIxArgs, SyncSolValueKeys,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{index_to_u32, SyncSolValueByMintFreeArgs};

use super::utils::ix_extend_with_sol_value_calculator_accounts;

pub fn sync_sol_value_ix_full<K: Into<SyncSolValueKeys>>(
    accounts: K,
    lst_index: usize,
    sol_value_calculator_accounts: &[AccountMeta],
    sol_value_calculator_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let lst_index = index_to_u32(lst_index)?;
    let mut ix = sync_sol_value_ix(accounts, SyncSolValueIxArgs { lst_index })?;
    ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        sol_value_calculator_accounts,
        sol_value_calculator_program_id,
    )
    .map_err(|_e| SControllerError::MathError)?;
    Ok(ix)
}

pub fn sync_sol_value_ix_by_mint_full<
    L: ReadonlyAccountData,
    M: ReadonlyAccountOwner + KeyedAccount,
>(
    free_args: SyncSolValueByMintFreeArgs<L, M>,
    sol_value_calculator_accounts: &[AccountMeta],
    sol_value_calculator_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let (keys, lst_index) = free_args.resolve()?;
    let ix = sync_sol_value_ix_full(
        keys,
        lst_index,
        sol_value_calculator_accounts,
        sol_value_calculator_program_id,
    )?;
    Ok(ix)
}
