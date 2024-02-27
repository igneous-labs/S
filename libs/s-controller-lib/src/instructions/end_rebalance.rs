use s_controller_interface::{
    end_rebalance_ix, EndRebalanceKeys, SControllerError, SControllerProgramIx,
    StartRebalanceIxArgs, StartRebalanceKeys, START_REBALANCE_IX_ACCOUNTS_LEN,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::EndRebalanceFromStartRebalanceKeys;

use super::utils::ix_extend_with_sol_value_calculator_accounts;

pub fn end_rebalance_ix_full(
    accounts: EndRebalanceKeys,
    dst_lst_calculator_accounts: &[AccountMeta],
    dst_lst_calculator_program_id: Pubkey,
) -> Result<Instruction, ProgramError> {
    let mut ix = end_rebalance_ix(accounts)?;
    ix_extend_with_sol_value_calculator_accounts(
        &mut ix,
        dst_lst_calculator_accounts,
        dst_lst_calculator_program_id,
    )
    .map_err(|_e| SControllerError::MathError)?;
    Ok(ix)
}

pub fn end_rebalance_ix_from_start_rebalance_ix(
    start_rebalance_ix: &Instruction,
) -> Result<Instruction, ProgramError> {
    let ix_data = SControllerProgramIx::deserialize(&start_rebalance_ix.data)?;
    let StartRebalanceIxArgs {
        src_lst_calc_accs, ..
    } = match ix_data {
        SControllerProgramIx::StartRebalance(args) => args,
        _ => return Err(ProgramError::InvalidInstructionData),
    };
    let dst_lst_suffix_start = START_REBALANCE_IX_ACCOUNTS_LEN + usize::from(src_lst_calc_accs);
    let dst_lst_suffix = start_rebalance_ix
        .accounts
        .get(dst_lst_suffix_start..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let start_rebalance_metas = start_rebalance_ix
        .accounts
        .get(..START_REBALANCE_IX_ACCOUNTS_LEN)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    // doing this default-then-init thing instead of a map() because AccountMeta is not Copy
    let mut start_rebalance_keys: [Pubkey; START_REBALANCE_IX_ACCOUNTS_LEN] = Default::default();
    for (i, meta) in start_rebalance_metas.iter().enumerate() {
        start_rebalance_keys[i] = meta.pubkey;
    }
    let end_rebalance_keys =
        EndRebalanceFromStartRebalanceKeys(&StartRebalanceKeys::from(start_rebalance_keys))
            .resolve();

    let mut ix = end_rebalance_ix(end_rebalance_keys)?;
    ix.accounts.extend(dst_lst_suffix.iter().cloned());
    Ok(ix)
}
