use s_controller_interface::{
    start_rebalance_ix_with_program_id, SControllerError, StartRebalanceIxArgs,
    StartRebalanceIxData, StartRebalanceKeys,
};
use solana_program::{instruction::Instruction, program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    index_to_u32, SrcDstLstIndexes, SrcDstLstSolValueCalcAccountSuffixes,
    StartRebalanceByMintsFreeArgs,
};

use super::{ix_extend_with_src_dst_sol_value_calculator_accounts, SrcDstLstSolValueCalcAccounts};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartRebalanceIxLstAmts {
    pub amount: u64,
    pub min_starting_src_lst: u64,
    pub max_starting_dst_lst: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartRebalanceIxFullArgs {
    pub src_lst_index: usize,
    pub dst_lst_index: usize,
    pub lst_amts: StartRebalanceIxLstAmts,
}

pub fn start_rebalance_ix_full(
    accounts: StartRebalanceKeys,
    args: StartRebalanceIxFullArgs,
    sol_val_calc_keys: SrcDstLstSolValueCalcAccounts,
) -> Result<Instruction, ProgramError> {
    start_rebalance_ix_full_for_prog(crate::program::ID, accounts, args, sol_val_calc_keys)
}

pub fn start_rebalance_ix_full_for_prog(
    program_id: Pubkey,
    accounts: StartRebalanceKeys,
    StartRebalanceIxFullArgs {
        src_lst_index,
        dst_lst_index,
        lst_amts:
            StartRebalanceIxLstAmts {
                amount,
                min_starting_src_lst,
                max_starting_dst_lst,
            },
    }: StartRebalanceIxFullArgs,
    sol_val_calc_keys: SrcDstLstSolValueCalcAccounts,
) -> Result<Instruction, ProgramError> {
    let src_lst_index = index_to_u32(src_lst_index)?;
    let dst_lst_index = index_to_u32(dst_lst_index)?;
    let mut ix = start_rebalance_ix_with_program_id(
        program_id,
        accounts,
        StartRebalanceIxArgs {
            src_lst_calc_accs: 0,
            src_lst_index,
            dst_lst_index,
            amount,
            min_starting_src_lst,
            max_starting_dst_lst,
        },
    )?;
    let extend_count =
        ix_extend_with_src_dst_sol_value_calculator_accounts(&mut ix, sol_val_calc_keys)
            .map_err(|_e| SControllerError::MathError)?;
    // TODO: better way to update src_lst_calc_accs than double serialization here
    let overwrite = &mut ix.data[..];
    StartRebalanceIxData(StartRebalanceIxArgs {
        src_lst_calc_accs: extend_count.src_lst,
        src_lst_index,
        dst_lst_index,
        amount,
        min_starting_src_lst,
        max_starting_dst_lst,
    })
    .serialize(overwrite)?;
    Ok(ix)
}

pub fn start_rebalance_ix_by_mints_full<
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    S: ReadonlyAccountData + ReadonlyAccountPubkey,
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
>(
    free_args: StartRebalanceByMintsFreeArgs<SM, DM, S, L>,
    lst_amts: StartRebalanceIxLstAmts,
    sol_val_calc_account_suffixes: SrcDstLstSolValueCalcAccountSuffixes,
) -> Result<Instruction, ProgramError> {
    let (
        start_rebalance_keys,
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
        program_ids,
    ) = free_args.resolve()?;
    start_rebalance_ix_full(
        start_rebalance_keys,
        StartRebalanceIxFullArgs {
            src_lst_index,
            dst_lst_index,
            lst_amts,
        },
        SrcDstLstSolValueCalcAccounts::new(program_ids, sol_val_calc_account_suffixes),
    )
}

pub fn start_rebalance_ix_by_mints_full_for_prog<
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    S: ReadonlyAccountData + ReadonlyAccountPubkey,
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
>(
    program_id: Pubkey,
    free_args: StartRebalanceByMintsFreeArgs<SM, DM, S, L>,
    lst_amts: StartRebalanceIxLstAmts,
    sol_val_calc_account_suffixes: SrcDstLstSolValueCalcAccountSuffixes,
) -> Result<Instruction, ProgramError> {
    let (
        start_rebalance_keys,
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
        program_ids,
    ) = free_args.resolve_for_prog(program_id)?;
    start_rebalance_ix_full_for_prog(
        program_id,
        start_rebalance_keys,
        StartRebalanceIxFullArgs {
            src_lst_index,
            dst_lst_index,
            lst_amts,
        },
        SrcDstLstSolValueCalcAccounts::new(program_ids, sol_val_calc_account_suffixes),
    )
}
