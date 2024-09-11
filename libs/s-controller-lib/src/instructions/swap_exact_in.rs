use s_controller_interface::{
    swap_exact_in_ix_with_program_id, SControllerError, SwapExactInIxArgs, SwapExactInIxData,
    SwapExactInKeys,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    index_to_u32, ix_extend_with_pricing_program_price_swap_accounts,
    ix_extend_with_src_dst_sol_value_calculator_accounts, SrcDstLstIndexes,
    SrcDstLstSolValueCalcAccountSuffixes, SrcDstLstSolValueCalcAccounts,
    SrcDstLstSolValueCalcExtendCount, SwapByMintsFreeArgs,
};

#[derive(Clone, Copy, Debug)]
pub struct SwapExactInIxFullArgs {
    pub src_lst_index: usize,
    pub dst_lst_index: usize,
    pub min_amount_out: u64,
    pub amount: u64,
}

pub fn swap_exact_in_ix_full(
    accounts: SwapExactInKeys,
    args: SwapExactInIxFullArgs,
    sol_val_calc_accounts: SrcDstLstSolValueCalcAccounts,
    pricing_program_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    swap_exact_in_ix_full_for_prog(
        crate::program::ID,
        accounts,
        args,
        sol_val_calc_accounts,
        pricing_program_accounts,
        pricing_program_id,
    )
}

pub fn swap_exact_in_ix_full_for_prog(
    program_id: Pubkey,
    accounts: SwapExactInKeys,
    SwapExactInIxFullArgs {
        src_lst_index,
        dst_lst_index,
        min_amount_out,
        amount,
    }: SwapExactInIxFullArgs,
    sol_val_calc_accounts: SrcDstLstSolValueCalcAccounts,
    pricing_program_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let src_lst_index = index_to_u32(src_lst_index)?;
    let dst_lst_index = index_to_u32(dst_lst_index)?;
    let mut ix = swap_exact_in_ix_with_program_id(
        program_id,
        accounts,
        SwapExactInIxArgs {
            // zeroes replaced by ix_extend below
            src_lst_value_calc_accs: 0,
            dst_lst_value_calc_accs: 0,
            src_lst_index,
            dst_lst_index,
            min_amount_out,
            amount,
        },
    )?;
    let SrcDstLstSolValueCalcExtendCount {
        src_lst: src_lst_value_calc_accs,
        dst_lst: dst_lst_value_calc_accs,
    } = ix_extend_with_src_dst_sol_value_calculator_accounts(&mut ix, sol_val_calc_accounts)
        .map_err(|_e| SControllerError::MathError)?;
    ix_extend_with_pricing_program_price_swap_accounts(
        &mut ix,
        pricing_program_accounts,
        pricing_program_id,
    )
    .map_err(|_e| SControllerError::MathError)?;
    // TODO: better way to update *_calc_accs than double serialization here
    let overwrite = &mut ix.data[..];
    SwapExactInIxData(SwapExactInIxArgs {
        src_lst_value_calc_accs,
        dst_lst_value_calc_accs,
        src_lst_index,
        dst_lst_index,
        min_amount_out,
        amount,
    })
    .serialize(overwrite)?;
    Ok(ix)
}

#[derive(Clone, Copy, Debug)]
pub struct SwapExactInAmounts {
    pub min_amount_out: u64,
    pub amount: u64,
}

pub fn swap_exact_in_ix_by_mint_full<
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    L: ReadonlyAccountData,
>(
    free_args: SwapByMintsFreeArgs<SM, DM, L>,
    SwapExactInAmounts {
        min_amount_out,
        amount,
    }: SwapExactInAmounts,
    src_dst_lst_sol_value_calc_account_suffixes: SrcDstLstSolValueCalcAccountSuffixes,
    pricing_program_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let (
        keys,
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
        src_dst_lst_sol_value_calc_program_ids,
    ) = free_args.resolve_exact_in()?;
    let ix = swap_exact_in_ix_full(
        keys,
        SwapExactInIxFullArgs {
            src_lst_index,
            dst_lst_index,
            min_amount_out,
            amount,
        },
        SrcDstLstSolValueCalcAccounts::new(
            src_dst_lst_sol_value_calc_program_ids,
            src_dst_lst_sol_value_calc_account_suffixes,
        ),
        pricing_program_accounts,
        pricing_program_id,
    )?;
    Ok(ix)
}

pub fn swap_exact_in_ix_by_mint_full_for_prog<
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    L: ReadonlyAccountData,
>(
    program_id: Pubkey,
    free_args: SwapByMintsFreeArgs<SM, DM, L>,
    SwapExactInAmounts {
        min_amount_out,
        amount,
    }: SwapExactInAmounts,
    src_dst_lst_sol_value_calc_account_suffixes: SrcDstLstSolValueCalcAccountSuffixes,
    pricing_program_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let (
        keys,
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
        src_dst_lst_sol_value_calc_program_ids,
    ) = free_args.resolve_exact_in_for_prog(program_id)?;
    let ix = swap_exact_in_ix_full_for_prog(
        program_id,
        keys,
        SwapExactInIxFullArgs {
            src_lst_index,
            dst_lst_index,
            min_amount_out,
            amount,
        },
        SrcDstLstSolValueCalcAccounts::new(
            src_dst_lst_sol_value_calc_program_ids,
            src_dst_lst_sol_value_calc_account_suffixes,
        ),
        pricing_program_accounts,
        pricing_program_id,
    )?;
    Ok(ix)
}
