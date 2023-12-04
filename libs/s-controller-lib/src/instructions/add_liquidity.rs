use borsh::BorshSerialize;
use s_controller_interface::{
    add_liquidity_ix, AddLiquidityIxArgs, AddLiquidityIxData, AddLiquidityKeys,
};
use solana_program::instruction::Instruction;

use crate::{
    ix_extend_with_pricing_program_price_lp_accounts, ix_extend_with_sol_value_calculator_accounts,
    try_from_int_err_to_io_err, AddRemoveLiquidityExtraAccounts,
};

#[derive(Clone, Copy, Debug)]
pub struct AddLiquidityIxFullArgs {
    pub lst_index: u32,
    pub amount: u64,
}

pub fn add_liquidity_ix_full<K: Into<AddLiquidityKeys>>(
    accounts: K,
    AddLiquidityIxFullArgs { lst_index, amount }: AddLiquidityIxFullArgs,
    AddRemoveLiquidityExtraAccounts {
        lst_calculator_program_id,
        pricing_program_id,
        lst_calculator_accounts,
        pricing_program_price_lp_accounts,
    }: AddRemoveLiquidityExtraAccounts,
) -> std::io::Result<Instruction> {
    let mut ix = add_liquidity_ix(
        accounts,
        AddLiquidityIxArgs {
            lst_value_calc_accs: 0,
            lst_index,
            amount,
        },
    )?;
    let lst_value_calc_accs = ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        lst_calculator_accounts,
        lst_calculator_program_id,
    )
    .map_err(try_from_int_err_to_io_err)?;
    ix_extend_with_pricing_program_price_lp_accounts(
        &mut ix,
        pricing_program_price_lp_accounts,
        pricing_program_id,
    )
    .map_err(try_from_int_err_to_io_err)?;
    // TODO: better way to update lst_value_calc_accs than double serialization here
    let mut overwrite = &mut ix.data[..];
    AddLiquidityIxData(AddLiquidityIxArgs {
        lst_value_calc_accs,
        lst_index,
        amount,
    })
    .serialize(&mut overwrite)?;
    Ok(ix)
}
