use std::num::TryFromIntError;

use s_controller_interface::SControllerError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[derive(Debug, Clone, Copy)]
pub struct SrcDstLstSolValueCalcProgramIds {
    pub src_lst_calculator_program_id: Pubkey,
    pub dst_lst_calculator_program_id: Pubkey,
}

/// Account suffixes should include the common interface account prefixes
/// but exclude the program ID
#[derive(Debug, Clone, Copy)]
pub struct SrcDstLstSolValueCalcAccountSuffixes<'me> {
    pub src_lst_calculator_accounts: &'me [AccountMeta],
    pub dst_lst_calculator_accounts: &'me [AccountMeta],
}

/// dst/src_lst_calculator_accounts should include the common interface account prefixes
/// but exclude the program ID
#[derive(Debug, Clone, Copy)]
pub struct SrcDstLstSolValueCalcAccounts<'me> {
    pub src_lst_calculator_program_id: Pubkey,
    pub dst_lst_calculator_program_id: Pubkey,
    pub src_lst_calculator_accounts: &'me [AccountMeta],
    pub dst_lst_calculator_accounts: &'me [AccountMeta],
}

impl<'me> SrcDstLstSolValueCalcAccounts<'me> {
    pub fn new(
        SrcDstLstSolValueCalcProgramIds {
            src_lst_calculator_program_id,
            dst_lst_calculator_program_id,
        }: SrcDstLstSolValueCalcProgramIds,
        SrcDstLstSolValueCalcAccountSuffixes {
            src_lst_calculator_accounts,
            dst_lst_calculator_accounts,
        }: SrcDstLstSolValueCalcAccountSuffixes<'me>,
    ) -> Self {
        Self {
            src_lst_calculator_program_id,
            dst_lst_calculator_program_id,
            src_lst_calculator_accounts,
            dst_lst_calculator_accounts,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SrcDstLstSolValueCalcExtendCount {
    pub src_lst: u8,
    pub dst_lst: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct AddRemoveLiquidityProgramIds {
    pub lst_calculator_program_id: Pubkey,
    pub pricing_program_id: Pubkey,
}

/// Account suffixes should include the common interface account prefixes
/// but exclude the program ID
#[derive(Debug, Clone, Copy)]
pub struct AddRemoveLiquidityAccountSuffixes<'me> {
    pub lst_calculator_accounts: &'me [AccountMeta],
    pub pricing_program_price_lp_accounts: &'me [AccountMeta],
}

/// lst_calculator_accounts & pricing_program_price_lp_accounts should include the common interface account prefixes
/// but exclude the program ID
#[derive(Debug, Clone, Copy)]
pub struct AddRemoveLiquidityExtraAccounts<'me> {
    pub lst_calculator_program_id: Pubkey,
    pub pricing_program_id: Pubkey,
    pub lst_calculator_accounts: &'me [AccountMeta],
    pub pricing_program_price_lp_accounts: &'me [AccountMeta],
}

impl<'me> AddRemoveLiquidityExtraAccounts<'me> {
    pub fn new(
        AddRemoveLiquidityProgramIds {
            lst_calculator_program_id,
            pricing_program_id,
        }: AddRemoveLiquidityProgramIds,
        AddRemoveLiquidityAccountSuffixes {
            lst_calculator_accounts,
            pricing_program_price_lp_accounts,
        }: AddRemoveLiquidityAccountSuffixes<'me>,
    ) -> Self {
        Self {
            lst_calculator_program_id,
            pricing_program_id,
            lst_calculator_accounts,
            pricing_program_price_lp_accounts,
        }
    }
}

/// sol_value_calculator_accounts should include common interface account prefixes
/// but exclude sol_value_calculator_program_id
/// Returns number of accounts added to the instruction's accounts array
pub fn ix_extend_with_sol_value_calculator_accounts(
    ix: &mut Instruction,
    sol_value_calculator_accounts: &[AccountMeta],
    sol_value_calculator_program_id: Pubkey,
) -> Result<u8, TryFromIntError> {
    account_metas_extend_with_sol_value_calculator_accounts(
        &mut ix.accounts,
        sol_value_calculator_accounts,
        sol_value_calculator_program_id,
    )
}

pub fn account_metas_extend_with_sol_value_calculator_accounts(
    accounts: &mut Vec<AccountMeta>,
    sol_value_calculator_accounts: &[AccountMeta],
    sol_value_calculator_program_id: Pubkey,
) -> Result<u8, TryFromIntError> {
    accounts.push(AccountMeta {
        pubkey: sol_value_calculator_program_id,
        is_signer: false,
        is_writable: false,
    });
    // exclude first account since that should be LST mint
    accounts.extend(sol_value_calculator_accounts.iter().skip(1).cloned());
    // n_accounts = len() - 1 + 1
    sol_value_calculator_accounts.len().try_into()
}

/// dst/src_lst_calculator_accounts should include common interface account prefixes
/// but exclude sol_value_calculator_program_id
pub fn ix_extend_with_src_dst_sol_value_calculator_accounts(
    ix: &mut Instruction,
    sol_val_calc_accounts: SrcDstLstSolValueCalcAccounts,
) -> Result<SrcDstLstSolValueCalcExtendCount, TryFromIntError> {
    account_metas_extend_with_src_dst_sol_value_calculator_accounts(
        &mut ix.accounts,
        sol_val_calc_accounts,
    )
}

pub fn account_metas_extend_with_src_dst_sol_value_calculator_accounts(
    accounts: &mut Vec<AccountMeta>,
    SrcDstLstSolValueCalcAccounts {
        src_lst_calculator_program_id,
        dst_lst_calculator_program_id,
        src_lst_calculator_accounts,
        dst_lst_calculator_accounts,
    }: SrcDstLstSolValueCalcAccounts,
) -> Result<SrcDstLstSolValueCalcExtendCount, TryFromIntError> {
    let src_lst = account_metas_extend_with_sol_value_calculator_accounts(
        accounts,
        src_lst_calculator_accounts,
        src_lst_calculator_program_id,
    )?;
    let dst_lst = account_metas_extend_with_sol_value_calculator_accounts(
        accounts,
        dst_lst_calculator_accounts,
        dst_lst_calculator_program_id,
    )?;
    Ok(SrcDstLstSolValueCalcExtendCount { src_lst, dst_lst })
}

// actually the same as ix_extend_with_sol_value_calculator_accounts
// since this interface also takes a single lst_mint prefix account
/// pricing_program_price_lp_accounts should include common interface account prefixes
/// but exclude pricing_program_id
/// Returns number of accounts added to the instruction's accounts array
pub fn ix_extend_with_pricing_program_price_lp_accounts(
    ix: &mut Instruction,
    pricing_program_price_lp_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<u8, TryFromIntError> {
    account_metas_extend_with_pricing_program_price_lp_accounts(
        &mut ix.accounts,
        pricing_program_price_lp_accounts,
        pricing_program_id,
    )
}

pub fn account_metas_extend_with_pricing_program_price_lp_accounts(
    accounts: &mut Vec<AccountMeta>,
    pricing_program_price_lp_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<u8, TryFromIntError> {
    accounts.push(AccountMeta {
        pubkey: pricing_program_id,
        is_signer: false,
        is_writable: false,
    });
    // exclude first account since that should be LST mint
    accounts.extend(pricing_program_price_lp_accounts.iter().skip(1).cloned());
    // n_accounts = len() - 1 + 1
    pricing_program_price_lp_accounts.len().try_into()
}

/// pricing_program_price_swap_accounts should include common interface account prefixes
/// but exclude pricing_program_id
/// Returns number of accounts added to the instruction's accounts array
pub fn ix_extend_with_pricing_program_price_swap_accounts(
    ix: &mut Instruction,
    pricing_program_price_swap_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<u8, SControllerError> {
    account_metas_extend_with_pricing_program_price_swap_accounts(
        &mut ix.accounts,
        pricing_program_price_swap_accounts,
        pricing_program_id,
    )
}

pub fn account_metas_extend_with_pricing_program_price_swap_accounts(
    accounts: &mut Vec<AccountMeta>,
    pricing_program_price_swap_accounts: &[AccountMeta],
    pricing_program_id: Pubkey,
) -> Result<u8, SControllerError> {
    accounts.push(AccountMeta {
        pubkey: pricing_program_id,
        is_signer: false,
        is_writable: false,
    });
    // exclude first 2 accounts since that should be input_lst_mint and output_lst_mint
    accounts.extend(pricing_program_price_swap_accounts.iter().skip(2).cloned());
    // n_accounts = len() - 2 + 1
    pricing_program_price_swap_accounts
        .len()
        .checked_sub(1)
        .and_then(|len| len.try_into().ok())
        .ok_or(SControllerError::MathError)
}

/// For conversion of u32 instruction args index types into usize.
/// Basically a wrapper around `try_into()`.
/// Reverse of [`index_to_u32`]
pub fn index_to_usize(ix_arg_index: u32) -> Result<usize, SControllerError> {
    ix_arg_index
        .try_into()
        .map_err(|_e| SControllerError::IndexTooLarge)
}

/// For converting usize index types to u32 to pack into instruction args.
/// Basically a wrapper around `try_into()`.
/// Reverse of [`index_to_usize`]
pub fn index_to_u32(ix_arg_index: usize) -> Result<u32, SControllerError> {
    ix_arg_index
        .try_into()
        .map_err(|_e| SControllerError::IndexTooLarge)
}
