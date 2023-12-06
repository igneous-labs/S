use s_controller_interface::{
    disable_lst_input_ix, enable_lst_input_ix, DisableLstInputIxArgs, EnableLstInputIxArgs,
};
use solana_program::{instruction::Instruction, program_error::ProgramError};
use solana_readonly_account::ReadonlyAccountData;

use crate::{index_to_u32, DisableEnableLstInputByMintFreeArgs};

pub fn disable_lst_input_ix_by_mint_full<S: ReadonlyAccountData, L: ReadonlyAccountData>(
    free_args: &DisableEnableLstInputByMintFreeArgs<S, L>,
) -> Result<Instruction, ProgramError> {
    let (keys, lst_index) = free_args.resolve_disable()?;
    let index = index_to_u32(lst_index)?;
    let ix = disable_lst_input_ix(keys, DisableLstInputIxArgs { index })?;
    Ok(ix)
}

pub fn enable_lst_input_ix_by_mint_full<S: ReadonlyAccountData, L: ReadonlyAccountData>(
    free_args: &DisableEnableLstInputByMintFreeArgs<S, L>,
) -> Result<Instruction, ProgramError> {
    let (keys, lst_index) = free_args.resolve_enable()?;
    let index = index_to_u32(lst_index)?;
    let ix = enable_lst_input_ix(keys, EnableLstInputIxArgs { index })?;
    Ok(ix)
}
