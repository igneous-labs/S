use borsh::BorshSerialize;
use s_controller_interface::{
    start_rebalance_ix, SControllerError, StartRebalanceIxArgs, StartRebalanceIxData,
    StartRebalanceKeys,
};
use solana_program::{instruction::Instruction, program_error::ProgramError};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{index_to_u32, SrcDstLstIndexes, StartRebalanceByMintsFreeArgs};

use super::{ix_extend_with_src_dst_sol_value_calculator_accounts, SrcDstLstSolValueCalcAccounts};

#[derive(Clone, Copy, Debug)]
pub struct StartRebalanceIxFullArgs {
    pub src_lst_index: usize,
    pub dst_lst_index: usize,
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
) -> Result<Instruction, ProgramError> {
    let src_lst_index = index_to_u32(src_lst_index)?;
    let dst_lst_index = index_to_u32(dst_lst_index)?;
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
            .map_err(|_e| SControllerError::MathError)?;
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

pub fn start_rebalance_ix_by_mints_full<
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    S: ReadonlyAccountData + ReadonlyAccountPubkey,
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
>(
    free_args: StartRebalanceByMintsFreeArgs<SM, DM, S, L>,
    amount: u64,
    sol_val_calc_accounts: SrcDstLstSolValueCalcAccounts,
) -> Result<Instruction, ProgramError> {
    let (
        start_rebalance_keys,
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
    ) = free_args.resolve()?;
    start_rebalance_ix_full(
        start_rebalance_keys,
        StartRebalanceIxFullArgs {
            src_lst_index,
            dst_lst_index,
            amount,
        },
        sol_val_calc_accounts,
    )
}
