use borsh::BorshSerialize;
use s_controller_interface::{
    swap_exact_in_ix, SControllerError, SwapExactInIxArgs, SwapExactInIxData, SwapExactInKeys,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    ix_extend_with_pricing_program_price_swap_accounts,
    ix_extend_with_src_dst_sol_value_calculator_accounts, SrcDstLstIndexes,
    SrcDstLstSolValueCalcAccounts, SrcDstLstSolValueCalcExtendCount, SwapExactInByMintFreeArgs,
};

#[derive(Clone, Copy, Debug)]
pub struct SwapExactInIxFullArgs {
    pub src_lst_index: u32,
    pub dst_lst_index: u32,
    pub min_amount_out: u64,
    pub amount: u64,
}

pub fn swap_exact_in_ix_full<K: Into<SwapExactInKeys>>(
    accounts: K,
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
    let mut ix = swap_exact_in_ix(
        accounts,
        SwapExactInIxArgs {
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
    let mut overwrite = &mut ix.data[..];
    SwapExactInIxData(SwapExactInIxArgs {
        src_lst_value_calc_accs,
        dst_lst_value_calc_accs,
        src_lst_index,
        dst_lst_index,
        min_amount_out,
        amount,
    })
    .serialize(&mut overwrite)?;
    Ok(ix)
}

#[derive(Clone, Copy, Debug)]
pub struct SwapExactInAmounts {
    pub min_amount_out: u64,
    pub amount: u64,
}

pub fn swap_exact_in_ix_by_mint_full<
    SM: ReadonlyAccountOwner + KeyedAccount,
    DM: ReadonlyAccountOwner + KeyedAccount,
    L: ReadonlyAccountData,
>(
    free_args: SwapExactInByMintFreeArgs<SM, DM, L>,
    SwapExactInAmounts {
        min_amount_out,
        amount,
    }: SwapExactInAmounts,
    sol_val_calc_accounts: SrcDstLstSolValueCalcAccounts,
    pricing_program_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let (
        keys,
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
    ) = free_args.resolve()?;
    let ix = swap_exact_in_ix_full(
        keys,
        SwapExactInIxFullArgs {
            src_lst_index: src_lst_index
                .try_into()
                .map_err(|_e| SControllerError::MathError)?,
            dst_lst_index: dst_lst_index
                .try_into()
                .map_err(|_e| SControllerError::MathError)?,
            min_amount_out,
            amount,
        },
        sol_val_calc_accounts,
        pricing_program_accounts,
        pricing_program_id,
    )?;
    Ok(ix)
}
