use s_controller_interface::{
    remove_liquidity_ix, RemoveLiquidityIxArgs, RemoveLiquidityIxData, RemoveLiquidityKeys,
    SControllerError,
};
use solana_program::{instruction::Instruction, program_error::ProgramError};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    index_to_u32, ix_extend_with_pricing_program_price_lp_accounts,
    ix_extend_with_sol_value_calculator_accounts, AddRemoveLiquidityAccountSuffixes,
    AddRemoveLiquidityExtraAccounts, RemoveLiquidityByMintFreeArgs,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RemoveLiquidityIxAmts {
    pub lp_token_amount: u64,
    pub min_lst_out: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RemoveLiquidityIxFullArgs {
    pub lst_index: usize,
    pub amts: RemoveLiquidityIxAmts,
}

pub fn remove_liquidity_ix_full(
    accounts: RemoveLiquidityKeys,
    RemoveLiquidityIxFullArgs {
        lst_index,
        amts:
            RemoveLiquidityIxAmts {
                lp_token_amount,
                min_lst_out,
            },
    }: RemoveLiquidityIxFullArgs,
    AddRemoveLiquidityExtraAccounts {
        lst_calculator_program_id,
        pricing_program_id,
        lst_calculator_accounts,
        pricing_program_price_lp_accounts,
    }: AddRemoveLiquidityExtraAccounts,
) -> Result<Instruction, ProgramError> {
    let lst_index = index_to_u32(lst_index)?;
    let mut ix = remove_liquidity_ix(
        accounts,
        RemoveLiquidityIxArgs {
            lst_value_calc_accs: 0,
            lst_index,
            lp_token_amount,
            min_lst_out,
        },
    )?;
    let lst_value_calc_accs = ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        lst_calculator_accounts,
        lst_calculator_program_id,
    )
    .map_err(|_e| SControllerError::MathError)?;
    ix_extend_with_pricing_program_price_lp_accounts(
        &mut ix,
        pricing_program_price_lp_accounts,
        pricing_program_id,
    )
    .map_err(|_e| SControllerError::MathError)?;
    // TODO: better way to update lst_value_calc_accs than double serialization here
    let mut overwrite = &mut ix.data[..];
    RemoveLiquidityIxData(RemoveLiquidityIxArgs {
        lst_value_calc_accs,
        lst_index,
        lp_token_amount,
        min_lst_out,
    })
    .serialize(&mut overwrite)?;
    Ok(ix)
}

pub fn remove_liquidity_ix_by_mint_full<
    S: ReadonlyAccountData,
    L: ReadonlyAccountData,
    M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
>(
    free_args: RemoveLiquidityByMintFreeArgs<S, L, M>,
    amts: RemoveLiquidityIxAmts,
    account_suffixes: AddRemoveLiquidityAccountSuffixes,
) -> Result<Instruction, ProgramError> {
    let (keys, lst_index, program_ids) = free_args.resolve()?;
    let ix = remove_liquidity_ix_full(
        keys,
        RemoveLiquidityIxFullArgs { lst_index, amts },
        AddRemoveLiquidityExtraAccounts::new(program_ids, account_suffixes),
    )?;
    Ok(ix)
}
