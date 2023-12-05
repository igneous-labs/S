use std::num::TryFromIntError;

use s_controller_interface::SControllerError;
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

#[derive(Debug, Clone, Copy)]
pub struct AddRemoveLiquidityExtraAccounts<'me> {
    pub lst_calculator_program_id: Pubkey,
    pub pricing_program_id: Pubkey,
    pub lst_calculator_accounts: &'me [AccountMeta],
    pub pricing_program_price_lp_accounts: &'me [AccountMeta],
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

// actually the same as ix_extend_with_sol_value_calculator_accounts
// since this interface also takes a single lst_mint prefix account
/// Returns number of accounts added to the instruction's accounts array
pub fn ix_extend_with_pricing_program_price_lp_accounts(
    ix: &mut Instruction,
    pricing_program_price_lp_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<u8, TryFromIntError> {
    ix.accounts.push(AccountMeta {
        pubkey: pricing_program_id,
        is_signer: false,
        is_writable: false,
    });
    // exclude first account since that should be LST mint
    ix.accounts
        .extend(pricing_program_price_lp_accounts.iter().skip(1).cloned());
    // n_accounts = len() - 1 + 1
    pricing_program_price_lp_accounts.len().try_into()
}

/// Returns number of accounts added to the instruction's accounts array
pub fn ix_extend_with_pricing_program_price_swap_accounts(
    ix: &mut Instruction,
    pricing_program_price_swap_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<u8, SControllerError> {
    ix.accounts.push(AccountMeta {
        pubkey: pricing_program_id,
        is_signer: false,
        is_writable: false,
    });
    // exclude first 2 accounts since that should be input_lst_mint and output_lst_mint
    ix.accounts
        .extend(pricing_program_price_swap_accounts.iter().skip(2).cloned());
    // n_accounts = len() - 2 + 1
    pricing_program_price_swap_accounts
        .len()
        .checked_sub(1)
        .and_then(|len| len.try_into().ok())
        .ok_or(SControllerError::MathError)
}
