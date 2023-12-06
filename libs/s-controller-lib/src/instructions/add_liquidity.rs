use borsh::BorshSerialize;
use s_controller_interface::{
    add_liquidity_ix, AddLiquidityIxArgs, AddLiquidityIxData, AddLiquidityKeys, SControllerError,
};
use solana_program::{instruction::Instruction, program_error::ProgramError};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    index_to_u32, ix_extend_with_pricing_program_price_lp_accounts,
    ix_extend_with_sol_value_calculator_accounts, AddLiquidityByMintFreeArgs,
    AddRemoveLiquidityExtraAccounts,
};

#[derive(Clone, Copy, Debug)]
pub struct AddLiquidityIxFullArgs {
    pub lst_index: usize,
    pub lst_amount: u64,
}

pub fn add_liquidity_ix_full<K: Into<AddLiquidityKeys>>(
    accounts: K,
    AddLiquidityIxFullArgs {
        lst_index,
        lst_amount,
    }: AddLiquidityIxFullArgs,
    AddRemoveLiquidityExtraAccounts {
        lst_calculator_program_id,
        pricing_program_id,
        lst_calculator_accounts,
        pricing_program_price_lp_accounts,
    }: AddRemoveLiquidityExtraAccounts,
) -> Result<Instruction, ProgramError> {
    let lst_index = index_to_u32(lst_index)?;
    let mut ix = add_liquidity_ix(
        accounts,
        AddLiquidityIxArgs {
            lst_value_calc_accs: 0,
            lst_index,
            lst_amount,
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
    AddLiquidityIxData(AddLiquidityIxArgs {
        lst_value_calc_accs,
        lst_index,
        lst_amount,
    })
    .serialize(&mut overwrite)?;
    Ok(ix)
}

pub fn add_liquidity_ix_by_mint_full<
    S: ReadonlyAccountData,
    L: ReadonlyAccountData,
    M: ReadonlyAccountOwner + KeyedAccount,
>(
    free_args: AddLiquidityByMintFreeArgs<S, L, M>,
    amount: u64,
    extra_accounts: AddRemoveLiquidityExtraAccounts,
) -> Result<Instruction, ProgramError> {
    let (keys, lst_index) = free_args.resolve()?;
    let ix = add_liquidity_ix_full(
        keys,
        AddLiquidityIxFullArgs {
            lst_index,
            lst_amount: amount,
        },
        extra_accounts,
    )?;
    Ok(ix)
}
