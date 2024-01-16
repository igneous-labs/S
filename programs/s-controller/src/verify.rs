//! Common verification functions used across multiple instruction processors

use s_controller_interface::{LstState, PoolState, SControllerError};
use s_controller_lib::{
    try_disable_pool_authority_list, try_find_element_in_list, SrcDstLstIndexes,
    SrcDstLstValueCalcAccs, U8Bool,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    account_traits::{
        GetLstMintAccountInfo, GetLstStateListAccountInfo, GetPoolStateAccountInfo,
        GetSrcDstLstMintAccountInfo, SrcDstLstMintAccountInfos,
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

pub fn verify_admin_or_disable_pool_authority(
    signer: Pubkey,
    pool_state: &PoolState,
    disable_pool_authority_list_acc: &AccountInfo,
) -> Result<(), ProgramError> {
    if signer != pool_state.admin {
        let data = disable_pool_authority_list_acc.try_borrow_data()?;
        let list = try_disable_pool_authority_list(&data)?;

        try_find_element_in_list(signer, list)
            .ok_or(SControllerError::InvalidDisablePoolAuthority)?;
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct VerifyLstSolValCalcCpiAccounts<'me, 'info> {
    pub lst_state_list: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
}

impl<'me, 'info, A> From<A> for VerifyLstSolValCalcCpiAccounts<'me, 'info>
where
    A: GetLstMintAccountInfo<'me, 'info> + GetLstStateListAccountInfo<'me, 'info>,
{
    fn from(ix_accounts: A) -> Self {
        Self {
            lst_mint: ix_accounts.get_lst_mint_account_info(),
            lst_state_list: ix_accounts.get_lst_state_list_account_info(),
        }
    }
}

pub fn verify_lst_sol_val_calc_cpi<'a, 'info>(
    VerifyLstSolValCalcCpiAccounts {
        lst_state_list,
        lst_mint,
    }: VerifyLstSolValCalcCpiAccounts<'a, 'info>,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
    lst_index: usize,
) -> Result<SolValueCalculatorCpi<'a, 'info>, ProgramError> {
    let cpi = SolValueCalculatorCpi::from_lst_mint_and_account_suffix_slice(
        lst_mint,
        accounts_suffix_slice,
    )?;
    cpi.verify_correct_sol_value_calculator_program(lst_state_list, lst_index)?;
    Ok(cpi)
}

#[derive(Clone, Copy, Debug)]
pub struct VerifyPricingSwapCpiAccounts<'me, 'info> {
    pub pool_state: &'me AccountInfo<'info>,
    pub src_dst_lst_mints: SrcDstLstMintAccountInfos<'me, 'info>,
}

impl<'me, 'info, A> From<A> for VerifyPricingSwapCpiAccounts<'me, 'info>
where
    A: GetPoolStateAccountInfo<'me, 'info> + GetSrcDstLstMintAccountInfo<'me, 'info>,
{
    fn from(ix_accounts: A) -> Self {
        Self {
            pool_state: ix_accounts.get_pool_state_account_info(),
            src_dst_lst_mints: ix_accounts.get_src_dst_lst_mints(),
        }
    }
}

pub fn verify_pricing_swap_cpi<'a, 'info>(
    VerifyPricingSwapCpiAccounts {
        pool_state,
        src_dst_lst_mints,
    }: VerifyPricingSwapCpiAccounts<'a, 'info>,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
) -> Result<PricingProgramPriceSwapCpi<'a, 'info>, ProgramError> {
    let pricing_program_cpi =
        PricingProgramPriceSwapCpi::from_src_dst_lst_mints_and_account_suffix_slice(
            src_dst_lst_mints,
            accounts_suffix_slice,
        )?;
    pricing_program_cpi.verify_correct_pricing_program(pool_state)?;
    Ok(pricing_program_cpi)
}

#[derive(Clone, Copy, Debug)]
pub struct VerifyPricingLpCpiAccounts<'me, 'info> {
    pub pool_state: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
}

impl<'me, 'info, A> From<A> for VerifyPricingLpCpiAccounts<'me, 'info>
where
    A: GetPoolStateAccountInfo<'me, 'info> + GetLstMintAccountInfo<'me, 'info>,
{
    fn from(ix_accounts: A) -> Self {
        Self {
            pool_state: ix_accounts.get_pool_state_account_info(),
            lst_mint: ix_accounts.get_lst_mint_account_info(),
        }
    }
}

pub fn verify_pricing_lp_cpi<'a, 'info>(
    VerifyPricingLpCpiAccounts {
        pool_state,
        lst_mint,
    }: VerifyPricingLpCpiAccounts<'a, 'info>,
    accounts_suffix_slice: &'a [AccountInfo<'info>],
) -> Result<PricingProgramPriceLpCpi<'a, 'info>, ProgramError> {
    let pricing_program_cpi = PricingProgramPriceLpCpi::from_lst_mint_and_account_suffix_slice(
        lst_mint,
        accounts_suffix_slice,
    )?;
    pricing_program_cpi.verify_correct_pricing_program(pool_state)?;
    Ok(pricing_program_cpi)
}

#[derive(Clone, Copy, Debug)]
pub struct VerifySrcDstLstSolValCalcCpiAccounts<'me, 'info> {
    pub lst_state_list: &'me AccountInfo<'info>,
    pub src_dst_lst_mints: SrcDstLstMintAccountInfos<'me, 'info>,
}

impl<'me, 'info, A> From<A> for VerifySrcDstLstSolValCalcCpiAccounts<'me, 'info>
where
    A: GetLstStateListAccountInfo<'me, 'info> + GetSrcDstLstMintAccountInfo<'me, 'info>,
{
    fn from(ix_accounts: A) -> Self {
        Self {
            lst_state_list: ix_accounts.get_lst_state_list_account_info(),
            src_dst_lst_mints: ix_accounts.get_src_dst_lst_mints(),
        }
    }
}

pub fn verify_src_dst_lst_sol_val_calc_cpis<'a, 'info>(
    VerifySrcDstLstSolValCalcCpiAccounts {
        lst_state_list,
        src_dst_lst_mints:
            SrcDstLstMintAccountInfos {
                src_lst_mint,
                dst_lst_mint,
            },
    }: VerifySrcDstLstSolValCalcCpiAccounts<'a, 'info>,
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
        VerifyLstSolValCalcCpiAccounts {
            lst_state_list,
            lst_mint: src_lst_mint,
        },
        src_suffix_slice,
        src_lst_index,
    )?;

    let dst_suffix_slice = accounts_suffix_slice
        .get(src_suffix_slice_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let dst_lst = verify_lst_sol_val_calc_cpi(
        VerifyLstSolValCalcCpiAccounts {
            lst_state_list,
            lst_mint: dst_lst_mint,
        },
        dst_suffix_slice,
        dst_lst_index,
    )?;

    Ok(SrcDstLstSolValueCalculatorCpis { src_lst, dst_lst })
}

#[derive(Clone, Copy, Debug)]
pub struct VerifyLpCpiAccounts<'me, 'info> {
    pub lst_state_list: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub lst_mint: &'me AccountInfo<'info>,
}

impl<'me, 'info, A> From<A> for VerifyLpCpiAccounts<'me, 'info>
where
    A: GetLstStateListAccountInfo<'me, 'info>
        + GetLstMintAccountInfo<'me, 'info>
        + GetPoolStateAccountInfo<'me, 'info>,
{
    fn from(ix_accounts: A) -> Self {
        Self {
            lst_state_list: ix_accounts.get_lst_state_list_account_info(),
            pool_state: ix_accounts.get_pool_state_account_info(),
            lst_mint: ix_accounts.get_lst_mint_account_info(),
        }
    }
}

pub fn verify_lp_cpis<'a, 'info>(
    VerifyLpCpiAccounts {
        lst_state_list,
        pool_state,
        lst_mint,
    }: VerifyLpCpiAccounts<'a, 'info>,
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
    let lst_cpi = verify_lst_sol_val_calc_cpi(
        VerifyLstSolValCalcCpiAccounts {
            lst_state_list,
            lst_mint,
        },
        lst_accounts_suffix_slice,
        lst_index,
    )?;

    let pricing_accounts_suffix_slice = accounts_suffix_slice
        .get(lst_accounts_suffix_slice_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let pricing_cpi = verify_pricing_lp_cpi(
        VerifyPricingLpCpiAccounts {
            pool_state,
            lst_mint,
        },
        pricing_accounts_suffix_slice,
    )?;

    Ok((lst_cpi, pricing_cpi))
}

#[derive(Clone, Copy, Debug)]
pub struct VerifySwapCpiAccounts<'me, 'info> {
    pub lst_state_list: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub src_dst_lst_mints: SrcDstLstMintAccountInfos<'me, 'info>,
}

impl<'me, 'info, A> From<A> for VerifySwapCpiAccounts<'me, 'info>
where
    A: GetLstStateListAccountInfo<'me, 'info>
        + GetSrcDstLstMintAccountInfo<'me, 'info>
        + GetPoolStateAccountInfo<'me, 'info>,
{
    fn from(ix_accounts: A) -> Self {
        Self {
            lst_state_list: ix_accounts.get_lst_state_list_account_info(),
            pool_state: ix_accounts.get_pool_state_account_info(),
            src_dst_lst_mints: ix_accounts.get_src_dst_lst_mints(),
        }
    }
}

pub fn verify_swap_cpis<'a, 'info>(
    VerifySwapCpiAccounts {
        lst_state_list,
        pool_state,
        src_dst_lst_mints,
    }: VerifySwapCpiAccounts<'a, 'info>,
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
        VerifySrcDstLstSolValCalcCpiAccounts {
            lst_state_list,
            src_dst_lst_mints,
        },
        src_dst_lst_suffix_slice,
        src_lst_value_calc_accs,
        src_dst_lst_indexes,
    )?;

    let pricing_program_accounts_suffix_slice = accounts_suffix_slice
        .get(src_dst_lst_suffix_slice_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let pricing_program_cpi = verify_pricing_swap_cpi(
        VerifyPricingSwapCpiAccounts {
            pool_state,
            src_dst_lst_mints,
        },
        pricing_program_accounts_suffix_slice,
    )?;

    Ok((sol_val_calc_cpis, pricing_program_cpi))
}

pub fn verify_swap_not_same_lst(
    src_lst_mint: &AccountInfo,
    dst_lst_mint: &AccountInfo,
) -> Result<(), SControllerError> {
    match src_lst_mint.key == dst_lst_mint.key {
        true => Err(SControllerError::SwapSameLst),
        false => Ok(()),
    }
}

fn verify_is_program(
    should_be_program: &AccountInfo,
    err: SControllerError,
) -> Result<(), SControllerError> {
    match should_be_program.executable {
        true => Ok(()),
        false => Err(err),
    }
}

pub fn verify_pricing_program_is_program(
    pricing_program: &AccountInfo,
) -> Result<(), SControllerError> {
    verify_is_program(pricing_program, SControllerError::FaultyPricingProgram)
}

pub fn verify_sol_value_calculator_is_program(
    sol_value_calculator_program: &AccountInfo,
) -> Result<(), SControllerError> {
    verify_is_program(
        sol_value_calculator_program,
        SControllerError::FaultySolValueCalculator,
    )
}
