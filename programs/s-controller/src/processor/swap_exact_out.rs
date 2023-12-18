use s_controller_interface::{
    swap_exact_out_verify_account_keys, swap_exact_out_verify_account_privileges, SControllerError,
    SwapExactOutAccounts, SwapExactOutIxArgs, SWAP_EXACT_OUT_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    calc_swap_protocol_fees, index_to_usize,
    program::{POOL_STATE_BUMP, POOL_STATE_SEED},
    try_lst_state_list, try_pool_state, CalcSwapProtocolFeesArgs, PoolStateAccount,
    SrcDstLstIndexes, SrcDstLstValueCalcAccs, SwapExactOutAmounts, SwapFreeArgs,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_token_lib::{
    token_account_balance, transfer_checked_decimal_agnostic_invoke,
    transfer_checked_decimal_agnostic_invoke_signed, TransferCheckedAccounts,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    account_traits::{DstLstPoolReservesOf, SrcLstPoolReservesOf},
    cpi::{PricingProgramIxArgs, PricingProgramPriceSwapCpi, SrcDstLstSolValueCalculatorCpis},
    verify::{
        verify_lst_input_not_disabled, verify_not_rebalancing_and_not_disabled, verify_swap_cpis,
        VerifySwapCpiAccounts,
    },
};

use super::{sync_sol_value_unchecked, SyncSolValueUncheckedAccounts};

pub fn process_swap_exact_out(accounts: &[AccountInfo], args: SwapExactOutIxArgs) -> ProgramResult {
    let (
        accounts,
        SwapExactOutAmounts {
            max_amount_in,
            amount,
        },
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
        SrcDstLstSolValueCalculatorCpis {
            src_lst: src_lst_cpi,
            dst_lst: dst_lst_cpi,
        },
        pricing_cpi,
    ) = verify_swap_exact_out(accounts, args)?;

    let src_sync_sol_value_accounts =
        SyncSolValueUncheckedAccounts::from(SrcLstPoolReservesOf(accounts));
    let dst_sync_sol_value_accounts =
        SyncSolValueUncheckedAccounts::from(DstLstPoolReservesOf(accounts));
    sync_sol_value_unchecked(src_sync_sol_value_accounts, src_lst_cpi, src_lst_index)?;
    sync_sol_value_unchecked(dst_sync_sol_value_accounts, dst_lst_cpi, dst_lst_index)?;

    let start_total_sol_value = accounts.pool_state.total_sol_value()?;

    let out_sol_value = dst_lst_cpi.invoke_lst_to_sol(amount)?.max;
    let in_sol_value = pricing_cpi.invoke_price_exact_out(PricingProgramIxArgs {
        amount,
        sol_value: out_sol_value,
    })?;
    let src_lst_in = src_lst_cpi.invoke_sol_to_lst(in_sol_value)?.max;

    if src_lst_in > max_amount_in {
        return Err(SControllerError::SlippageToleranceExceeded.into());
    }
    if src_lst_in == 0 {
        return Err(SControllerError::ZeroValue.into());
    }

    let trading_protocol_fee_bps = accounts.pool_state.trading_protocol_fee_bps()?;
    let to_protocol_fees_lst_amount = calc_swap_protocol_fees(CalcSwapProtocolFeesArgs {
        in_sol_value,
        out_sol_value,
        dst_lst_out: amount,
        trading_protocol_fee_bps,
    })?;

    let total_dst_lst_out = amount
        .checked_add(to_protocol_fees_lst_amount)
        .ok_or(SControllerError::MathError)?;
    if total_dst_lst_out > token_account_balance(accounts.dst_pool_reserves)? {
        return Err(SControllerError::NotEnoughLiquidity.into());
    }

    transfer_checked_decimal_agnostic_invoke(
        TransferCheckedAccounts {
            from: accounts.src_lst_acc,
            to: accounts.src_pool_reserves,
            token_program: accounts.src_lst_token_program,
            authority: accounts.signer,
            mint: accounts.src_lst_mint,
        },
        src_lst_in,
    )?;
    transfer_checked_decimal_agnostic_invoke_signed(
        TransferCheckedAccounts {
            from: accounts.dst_pool_reserves,
            to: accounts.protocol_fee_accumulator,
            token_program: accounts.dst_lst_token_program,
            authority: accounts.pool_state,
            mint: accounts.dst_lst_mint,
        },
        to_protocol_fees_lst_amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;
    transfer_checked_decimal_agnostic_invoke_signed(
        TransferCheckedAccounts {
            from: accounts.dst_pool_reserves,
            to: accounts.dst_lst_acc,
            token_program: accounts.dst_lst_token_program,
            authority: accounts.pool_state,
            mint: accounts.dst_lst_mint,
        },
        amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    sync_sol_value_unchecked(src_sync_sol_value_accounts, src_lst_cpi, src_lst_index)?;
    sync_sol_value_unchecked(dst_sync_sol_value_accounts, dst_lst_cpi, dst_lst_index)?;

    let end_total_sol_value = accounts.pool_state.total_sol_value()?;
    if end_total_sol_value < start_total_sol_value {
        return Err(SControllerError::PoolWouldLoseSolValue.into());
    }

    Ok(())
}

fn verify_swap_exact_out<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    SwapExactOutIxArgs {
        src_lst_value_calc_accs,
        dst_lst_value_calc_accs,
        src_lst_index,
        dst_lst_index,
        max_amount_in,
        amount,
    }: SwapExactOutIxArgs,
) -> Result<
    (
        SwapExactOutAccounts<'a, 'info>,
        SwapExactOutAmounts,
        SrcDstLstIndexes,
        SrcDstLstSolValueCalculatorCpis<'a, 'info>,
        PricingProgramPriceSwapCpi<'a, 'info>,
    ),
    ProgramError,
> {
    if amount == 0 || max_amount_in == 0 {
        return Err(SControllerError::ZeroValue.into());
    }

    let src_lst_index = index_to_usize(src_lst_index)?;
    let dst_lst_index = index_to_usize(dst_lst_index)?;

    let actual: SwapExactOutAccounts = load_accounts(accounts)?;

    let free_args = SwapFreeArgs {
        signer: *actual.signer.key,
        src_lst_acc: *actual.src_lst_acc.key,
        dst_lst_acc: *actual.dst_lst_acc.key,
        src_lst_index,
        dst_lst_index,
        src_lst_mint: actual.src_lst_mint,
        dst_lst_mint: actual.dst_lst_mint,
        lst_state_list: actual.lst_state_list,
    };
    let expected = free_args.resolve_exact_out()?;

    swap_exact_out_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    swap_exact_out_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    let lst_state_list_bytes = actual.lst_state_list.try_borrow_data()?;
    let lst_state_list = try_lst_state_list(&lst_state_list_bytes)?;
    let src_lst_state = lst_state_list[src_lst_index];
    verify_lst_input_not_disabled(&src_lst_state)?;

    let src_dst_lst_indexes = SrcDstLstIndexes {
        src_lst_index,
        dst_lst_index,
    };
    let src_dst_lst_value_calc_accs = SrcDstLstValueCalcAccs {
        src_lst_value_calc_accs,
        dst_lst_value_calc_accs,
    };

    let accounts_suffix_slice = accounts
        .get(SWAP_EXACT_OUT_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let (src_dst_cpis, pricing_cpi) = verify_swap_cpis(
        VerifySwapCpiAccounts::from(actual),
        accounts_suffix_slice,
        src_dst_lst_value_calc_accs,
        src_dst_lst_indexes,
    )?;

    Ok((
        actual,
        SwapExactOutAmounts {
            max_amount_in,
            amount,
        },
        src_dst_lst_indexes,
        src_dst_cpis,
        pricing_cpi,
    ))
}
