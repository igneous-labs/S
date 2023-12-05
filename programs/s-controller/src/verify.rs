//! Common verification functions used across multiple instruction processors

use s_controller_interface::{LstState, PoolState, SControllerError};
use s_controller_lib::{SrcDstLstIndexes, SrcDstLstValueCalcAccs, U8Bool};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::{
    account_traits::{
        DstLstMintOf, GetLstMintAccountInfo, GetLstStateListAccountInfo, GetPoolStateAccountInfo,
        GetSrcDstLstMintAccountInfo, SrcLstMintOf,
    },
    cpi::{
        PricingProgramPriceLpCpi, PricingProgramPriceSwapCpi, SolValueCalculatorCpi,
        SrcDstLstSolValueCalculatorCpis,
    },
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

pub fn verify_lst_sol_val_calc_cpi<
    'a,
    'info,
    A: GetLstStateListAccountInfo<'a, 'info> + GetLstMintAccountInfo<'a, 'info>,
>(
    base_accounts: A,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
    lst_index: usize,
) -> Result<SolValueCalculatorCpi<'a, 'info>, ProgramError> {
    let cpi = SolValueCalculatorCpi::from_ix_accounts(&base_accounts, accounts_suffix_slice)?;
    cpi.verify_correct_sol_value_calculator_program(base_accounts, lst_index)?;
    Ok(cpi)
}

pub fn verify_pricing_swap_cpi<
    'a,
    'info,
    A: GetPoolStateAccountInfo<'a, 'info> + GetSrcDstLstMintAccountInfo<'a, 'info>,
>(
    base_accounts: A,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
) -> Result<PricingProgramPriceSwapCpi<'a, 'info>, ProgramError> {
    let pricing_program_cpi =
        PricingProgramPriceSwapCpi::from_ix_accounts(&base_accounts, accounts_suffix_slice)?;
    pricing_program_cpi.verify_correct_pricing_program(base_accounts)?;
    Ok(pricing_program_cpi)
}

pub fn verify_pricing_lp_cpi<
    'a,
    'info,
    A: GetPoolStateAccountInfo<'a, 'info> + GetLstMintAccountInfo<'a, 'info>,
>(
    base_accounts: A,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
) -> Result<PricingProgramPriceLpCpi<'a, 'info>, ProgramError> {
    let pricing_program_cpi =
        PricingProgramPriceLpCpi::from_ix_accounts(&base_accounts, accounts_suffix_slice)?;
    pricing_program_cpi.verify_correct_pricing_program(base_accounts)?;
    Ok(pricing_program_cpi)
}

pub fn verify_src_dst_lst_sol_val_calc_cpis<
    'a,
    'info,
    A: GetSrcDstLstMintAccountInfo<'a, 'info> + GetLstStateListAccountInfo<'a, 'info>,
>(
    base_accounts: A,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
    src_lst_value_calc_accs: u8,
    SrcDstLstIndexes {
        src_lst_index,
        dst_lst_index,
    }: SrcDstLstIndexes,
) -> Result<SrcDstLstSolValueCalculatorCpis<'a, 'info>, ProgramError> {
    let src_suffix_slice_end = src_lst_value_calc_accs.into();

    let src_suffix_slice = accounts_suffix_slice
        .get(..src_suffix_slice_end)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let src_lst = verify_lst_sol_val_calc_cpi(
        SrcLstMintOf(&base_accounts),
        src_suffix_slice,
        src_lst_index,
    )?;

    let dst_suffix_slice = accounts_suffix_slice
        .get(src_suffix_slice_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let dst_lst = verify_lst_sol_val_calc_cpi(
        DstLstMintOf(&base_accounts),
        dst_suffix_slice,
        dst_lst_index,
    )?;

    Ok(SrcDstLstSolValueCalculatorCpis { src_lst, dst_lst })
}

pub fn verify_lp_cpis<
    'a,
    'info,
    A: GetLstStateListAccountInfo<'a, 'info>
        + GetLstMintAccountInfo<'a, 'info>
        + GetPoolStateAccountInfo<'a, 'info>,
>(
    base_accounts: A,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
    lst_value_calc_accs: u8,
    lst_index: usize,
) -> Result<
    (
        SolValueCalculatorCpi<'a, 'info>,
        PricingProgramPriceLpCpi<'a, 'info>,
    ),
    ProgramError,
> {
    let lst_accounts_suffix_slice_end: usize = lst_value_calc_accs.into();

    let lst_accounts_suffix_slice = accounts_suffix_slice
        .get(..lst_accounts_suffix_slice_end)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let lst_cpi =
        verify_lst_sol_val_calc_cpi(&base_accounts, lst_accounts_suffix_slice, lst_index)?;

    let pricing_accounts_suffix_slice = accounts_suffix_slice
        .get(lst_accounts_suffix_slice_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let pricing_cpi = verify_pricing_lp_cpi(base_accounts, pricing_accounts_suffix_slice)?;

    Ok((lst_cpi, pricing_cpi))
}

pub fn verify_swap_cpis<
    'a,
    'info,
    A: GetSrcDstLstMintAccountInfo<'a, 'info>
        + GetLstStateListAccountInfo<'a, 'info>
        + GetPoolStateAccountInfo<'a, 'info>,
>(
    base_accounts: A,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
    SrcDstLstValueCalcAccs {
        src_lst_value_calc_accs,
        dst_lst_value_calc_accs,
    }: SrcDstLstValueCalcAccs,
    src_dst_lst_indexes: SrcDstLstIndexes,
) -> Result<
    (
        SrcDstLstSolValueCalculatorCpis<'a, 'info>,
        PricingProgramPriceSwapCpi<'a, 'info>,
    ),
    ProgramError,
> {
    let src_lst_value_calc_accs_usize: usize = src_lst_value_calc_accs.into();
    let dst_lst_value_calc_accs_usize: usize = dst_lst_value_calc_accs.into();
    // no overflow, u8
    let src_dst_lst_suffix_slice_end =
        src_lst_value_calc_accs_usize + dst_lst_value_calc_accs_usize;
    let src_dst_lst_suffix_slice = accounts_suffix_slice
        .get(..src_dst_lst_suffix_slice_end)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let sol_val_calc_cpis = verify_src_dst_lst_sol_val_calc_cpis(
        &base_accounts,
        src_dst_lst_suffix_slice,
        src_lst_value_calc_accs,
        src_dst_lst_indexes,
    )?;

    let pricing_program_accounts_suffix_slice = accounts_suffix_slice
        .get(src_dst_lst_suffix_slice_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let pricing_program_cpi =
        verify_pricing_swap_cpi(base_accounts, pricing_program_accounts_suffix_slice)?;

    Ok((sol_val_calc_cpis, pricing_program_cpi))
}
