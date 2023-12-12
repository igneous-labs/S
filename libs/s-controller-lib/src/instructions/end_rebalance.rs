use s_controller_interface::{
    end_rebalance_ix, EndRebalanceKeys, SControllerError, StartRebalanceKeys,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::EndRebalanceFromStartRebalanceKeys;

use super::utils::ix_extend_with_sol_value_calculator_accounts;

pub fn end_rebalance_ix_full<K: Into<EndRebalanceKeys>>(
    accounts: K,
    dst_lst_calculator_accounts: &[AccountMeta],
    dst_lst_calculator_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let mut ix = end_rebalance_ix(accounts)?;
    ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        dst_lst_calculator_accounts,
        dst_lst_calculator_program_id,
    )
    .map_err(|_e| SControllerError::MathError)?;
    Ok(ix)
}

pub fn end_rebalance_ix_from_start_rebalance_keys(
    start_rebalance_keys: &StartRebalanceKeys,
    dst_lst_calculator_accounts: &[AccountMeta],
    dst_lst_calculator_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = EndRebalanceFromStartRebalanceKeys(start_rebalance_keys).resolve();
    end_rebalance_ix_full(
        accounts,
        dst_lst_calculator_accounts,
        dst_lst_calculator_program_id,
    )
}
