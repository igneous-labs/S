use borsh::BorshSerialize;
use s_controller_interface::{
    start_rebalance_ix, StartRebalanceIxArgs, StartRebalanceIxData, StartRebalanceKeys,
};
use solana_program::instruction::Instruction;

use super::{
    ix_extend_with_src_dst_sol_value_calculator_accounts, try_from_int_err_to_io_err,
    SrcDstLstSolValueCalcAccounts,
};

#[derive(Clone, Copy, Debug)]
pub struct StartRebalanceIxFullArgs {
    pub src_lst_index: u32,
    pub dst_lst_index: u32,
    pub amount: u64,
}

pub fn start_rebalance_ix_full<K: Into<StartRebalanceKeys>>(
    accounts: K,
    StartRebalanceIxFullArgs {
        src_lst_index,
        dst_lst_index,
        amount,
    }: StartRebalanceIxFullArgs,
    sol_val_calc_keys: SrcDstLstSolValueCalcAccounts,
) -> std::io::Result<Instruction> {
    let mut ix = start_rebalance_ix(
        accounts,
        StartRebalanceIxArgs {
            src_lst_calc_accs: 0,
            src_lst_index,
            dst_lst_index,
            amount,
        },
    )?;
    let extend_count =
        ix_extend_with_src_dst_sol_value_calculator_accounts(&mut ix, sol_val_calc_keys)
            .map_err(try_from_int_err_to_io_err)?;
    // TODO: better way to update src_lst_calc_accs than double serialization here
    let mut overwrite = &mut ix.data[..];
    StartRebalanceIxData(StartRebalanceIxArgs {
        src_lst_calc_accs: extend_count.src_lst,
        src_lst_index,
        dst_lst_index,
        amount,
    })
    .serialize(&mut overwrite)?;
    Ok(ix)
}
