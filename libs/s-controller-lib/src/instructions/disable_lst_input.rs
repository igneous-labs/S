use s_controller_interface::{disable_lst_input_ix, DisableLstInputIxArgs};
use solana_program::{instruction::Instruction, program_error::ProgramError};
use solana_readonly_account::ReadonlyAccountData;

use crate::{index_to_u32, DisableLstInputByMintFreeArgs};

pub fn disable_lst_input_ix_by_mint_full<S: ReadonlyAccountData, L: ReadonlyAccountData>(
    free_args: DisableLstInputByMintFreeArgs<S, L>,
) -> Result<Instruction, ProgramError> {
    let (keys, lst_index) = free_args.resolve()?;
    let index = index_to_u32(lst_index)?;
    let ix = disable_lst_input_ix(keys, DisableLstInputIxArgs { index })?;
    Ok(ix)
}
