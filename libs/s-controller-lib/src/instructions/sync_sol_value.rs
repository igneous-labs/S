use s_controller_interface::{
    sync_sol_value_ix, SControllerError, SyncSolValueIxArgs, SyncSolValueKeys,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::SyncSolValueByMintFreeArgs;

use super::utils::ix_extend_with_sol_value_calculator_accounts;

pub fn sync_sol_value_ix_full<K: Into<SyncSolValueKeys>, A: Into<SyncSolValueIxArgs>>(
    accounts: K,
    args: A,
    sol_value_calculator_accounts: &[AccountMeta],
    sol_value_calculator_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let mut ix = sync_sol_value_ix(accounts, args)?;
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
    let (keys, ix_args) = free_args.resolve()?;
    let ix = sync_sol_value_ix_full(
        keys,
        ix_args,
        sol_value_calculator_accounts,
        sol_value_calculator_program_id,
    )?;
    Ok(ix)
}
