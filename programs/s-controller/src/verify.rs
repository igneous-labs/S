//! Common verification functions used across multiple instruction processors

use s_controller_interface::{
    LstState, PoolState, SControllerError, SWAP_EXACT_IN_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{SrcDstLstIndexes, SrcDstLstValueCalcAccs, U8Bool};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::{
    account_traits::{
        DstLstMintOf, GetLstStateListAccountInfo, GetPoolStateAccountInfo,
        GetSrcDstLstMintAccountInfo, SrcLstMintOf,
    },
    cpi::{PricingProgramPriceSwapCpi, SolValueCalculatorCpi, SrcDstLstSolValueCalculatorCpis},
};

pub const fn verify_not_rebalancing_and_not_disabled(
    pool_state: &PoolState,
) -> Result<(), SControllerError> {
    if U8Bool(pool_state.is_rebalancing).is_true() {
        return Err(SControllerError::PoolRebalancing);
    }
    if U8Bool(pool_state.is_disabled).is_true() {
        return Err(SControllerError::PoolDisabled);
    }
    Ok(())
}

pub const fn verify_lst_input_not_disabled(lst_state: &LstState) -> Result<(), SControllerError> {
    if U8Bool(lst_state.is_input_disabled).is_true() {
        return Err(SControllerError::LstInputDisabled);
    }
    Ok(())
}

pub fn verify_swap_cpis<
    'a,
    'info,
    A: GetSrcDstLstMintAccountInfo<'a, 'info>
        + GetLstStateListAccountInfo<'a, 'info>
        + GetPoolStateAccountInfo<'a, 'info>,
>(
    ix_accounts_full: &'a [AccountInfo<'info>],
    base_swap_accounts: &A,
    SrcDstLstValueCalcAccs {
        src_lst_value_calc_accs,
        dst_lst_value_calc_accs,
    }: SrcDstLstValueCalcAccs,
    SrcDstLstIndexes {
        src_lst_index,
        dst_lst_index,
    }: SrcDstLstIndexes,
) -> Result<
    (
        SrcDstLstSolValueCalculatorCpis<'a, 'info>,
        PricingProgramPriceSwapCpi<'a, 'info>,
    ),
    ProgramError,
> {
    // const asserts SWAP_EXACT_IN_IX_ACCOUNTS_LEN == SWAP_EXACT_OUT_IX_ACCOUNTS_LEN in _lib
    let src_lst_accounts_suffix_end = SWAP_EXACT_IN_IX_ACCOUNTS_LEN
        .checked_add(src_lst_value_calc_accs.into())
        .ok_or(SControllerError::MathError)?;
    let src_lst_accounts_suffix_slice = ix_accounts_full
        .get(SWAP_EXACT_IN_IX_ACCOUNTS_LEN..src_lst_accounts_suffix_end)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let src_lst_cpi = SolValueCalculatorCpi::from_ix_accounts(
        SrcLstMintOf(base_swap_accounts),
        src_lst_accounts_suffix_slice,
    )?;
    src_lst_cpi.verify_correct_sol_value_calculator_program(base_swap_accounts, src_lst_index)?;

    let dst_lst_accounts_suffix_slice_end = src_lst_accounts_suffix_end
        .checked_add(dst_lst_value_calc_accs.into())
        .ok_or(SControllerError::MathError)?;
    let dst_lst_accounts_suffix_slice = ix_accounts_full
        .get(src_lst_accounts_suffix_end..dst_lst_accounts_suffix_slice_end)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let dst_lst_cpi = SolValueCalculatorCpi::from_ix_accounts(
        DstLstMintOf(base_swap_accounts),
        dst_lst_accounts_suffix_slice,
    )?;
    dst_lst_cpi.verify_correct_sol_value_calculator_program(base_swap_accounts, dst_lst_index)?;

    let pricing_program_accounts_suffix_slice = ix_accounts_full
        .get(dst_lst_accounts_suffix_slice_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let pricing_program_cpi = PricingProgramPriceSwapCpi::from_ix_accounts(
        base_swap_accounts,
        pricing_program_accounts_suffix_slice,
    )?;
    pricing_program_cpi.verify_correct_pricing_program(base_swap_accounts)?;

    Ok((
        SrcDstLstSolValueCalculatorCpis {
            src_lst: src_lst_cpi,
            dst_lst: dst_lst_cpi,
        },
        pricing_program_cpi,
    ))
}
