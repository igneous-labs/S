use s_controller_interface::{end_rebalance_ix, EndRebalanceIxArgs, EndRebalanceKeys};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::utils::ix_extend_with_sol_value_calculator_accounts;

pub fn end_rebalance_ix_full<
    K: Into<EndRebalanceKeys>,
    D: Into<[AccountMeta; N]>,
    const N: usize,
>(
    accounts: K,
    dst_lst_calculator_keys: D,
    dst_lst_calculator_program_id: Pubkey,
) -> std::io::Result<Instruction> {
    let mut ix = end_rebalance_ix(accounts, EndRebalanceIxArgs {})?;
    ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        dst_lst_calculator_keys,
        dst_lst_calculator_program_id,
    );
    Ok(ix)
}
