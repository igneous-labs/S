use s_controller_interface::{
    end_rebalance_ix, EndRebalanceIxArgs, EndRebalanceKeys, StartRebalanceKeys,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::EndRebalanceFromStartRebalanceKeys;

use super::utils::{ix_extend_with_sol_value_calculator_accounts, try_from_int_err_to_io_err};

pub fn end_rebalance_ix_full<K: Into<EndRebalanceKeys>>(
    accounts: K,
    dst_lst_calculator_accounts: &[AccountMeta],
    dst_lst_calculator_program_id: Pubkey,
) -> std::io::Result<Instruction> {
    let mut ix = end_rebalance_ix(accounts, EndRebalanceIxArgs {})?;
    ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        dst_lst_calculator_accounts,
        dst_lst_calculator_program_id,
    )
    .map_err(try_from_int_err_to_io_err)?;
    Ok(ix)
}

pub fn end_rebalance_ix_from_start_rebalance_keys(
    start_rebalance_keys: &StartRebalanceKeys,
    dst_lst_calculator_accounts: &[AccountMeta],
    dst_lst_calculator_program_id: Pubkey,
) -> std::io::Result<Instruction> {
    let accounts = EndRebalanceFromStartRebalanceKeys(start_rebalance_keys).resolve();
    end_rebalance_ix_full(
        accounts,
        dst_lst_calculator_accounts,
        dst_lst_calculator_program_id,
    )
}
