use std::num::TryFromIntError;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[derive(Debug, Clone, Copy)]
pub struct SrcDstLstSolValueCalcAccounts<'me> {
    pub src_lst_calculator_program_id: Pubkey,
    pub dst_lst_calculator_program_id: Pubkey,
    pub src_lst_calculator_accounts: &'me [AccountMeta],
    pub dst_lst_calculator_accounts: &'me [AccountMeta],
}

#[derive(Debug, Clone, Copy)]
pub struct SrcDstLstSolValueCalcExtendCount {
    pub src_lst: u8,
    pub dst_lst: u8,
}

/// Returns number of accounts added to the instruction's accounts array
pub fn ix_extend_with_sol_value_calculator_accounts(
    ix: &mut Instruction,
    sol_value_calculator_accounts: &[AccountMeta],
    sol_value_calculator_program_id: Pubkey,
) -> Result<u8, TryFromIntError> {
    ix.accounts.push(AccountMeta {
        pubkey: sol_value_calculator_program_id,
        is_signer: false,
        is_writable: false,
    });
    // exclude first account since that should be LST mint
    ix.accounts
        .extend(sol_value_calculator_accounts.iter().skip(1).cloned());
    // n_accounts = len() - 1 + 1
    sol_value_calculator_accounts.len().try_into()
}

pub fn ix_extend_with_src_dst_sol_value_calculator_accounts(
    ix: &mut Instruction,
    SrcDstLstSolValueCalcAccounts {
        src_lst_calculator_program_id,
        dst_lst_calculator_program_id,
        src_lst_calculator_accounts,
        dst_lst_calculator_accounts,
    }: SrcDstLstSolValueCalcAccounts,
) -> Result<SrcDstLstSolValueCalcExtendCount, TryFromIntError> {
    let src_lst = ix_extend_with_sol_value_calculator_accounts(
        ix,
        src_lst_calculator_accounts,
        src_lst_calculator_program_id,
    )?;
    let dst_lst = ix_extend_with_sol_value_calculator_accounts(
        ix,
        dst_lst_calculator_accounts,
        dst_lst_calculator_program_id,
    )?;
    Ok(SrcDstLstSolValueCalcExtendCount { src_lst, dst_lst })
}

pub fn try_from_int_err_to_io_err(e: TryFromIntError) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e)
}
