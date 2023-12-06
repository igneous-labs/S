use s_controller_interface::{
    swap_exact_in_verify_account_keys, swap_exact_in_verify_account_privileges, SControllerError,
    SwapExactInAccounts, SwapExactInIxArgs, SWAP_EXACT_IN_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    calc_swap_protocol_fees, index_to_usize,
    program::{POOL_STATE_BUMP, POOL_STATE_SEED},
    try_lst_state_list, try_pool_state, CalcSwapProtocolFeesArgs, PoolStateAccount,
    SrcDstLstIndexes, SrcDstLstValueCalcAccs, SwapExactInAmounts, SwapFreeArgs,
};
use sanctum_onchain_utils::{
    token_program::{transfer_tokens, transfer_tokens_signed, TransferTokensAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use sanctum_utils::token::token_account_balance_program_agnostic;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    account_traits::{DstLstPoolReservesOf, SrcLstPoolReservesOf},
    cpi::{PricingProgramIxArgs, PricingProgramPriceSwapCpi, SrcDstLstSolValueCalculatorCpis},
    processor::sync_sol_value_unchecked,
    verify::{
        verify_lst_input_not_disabled, verify_not_rebalancing_and_not_disabled, verify_swap_cpis,
    },
};

pub fn process_swap_exact_in(accounts: &[AccountInfo], args: SwapExactInIxArgs) -> ProgramResult {
    let (
        accounts,
        SwapExactInAmounts {
            min_amount_out,
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
    ) = verify_swap_exact_in(accounts, args)?;

    sync_sol_value_unchecked(SrcLstPoolReservesOf(&accounts), src_lst_cpi, src_lst_index)?;
    sync_sol_value_unchecked(DstLstPoolReservesOf(&accounts), dst_lst_cpi, dst_lst_index)?;

    let start_total_sol_value = accounts.pool_state.total_sol_value()?;

    let in_sol_value = src_lst_cpi.invoke_lst_to_sol(amount)?;
    let out_sol_value = pricing_cpi.invoke_price_exact_in(PricingProgramIxArgs {
        amount,
        sol_value: in_sol_value,
    })?;
    let dst_lst_out = dst_lst_cpi.invoke_sol_to_lst(out_sol_value)?;

    if dst_lst_out < min_amount_out {
        return Err(SControllerError::SlippageToleranceExceeded.into());
    }

    let trading_protocol_fee_bps = accounts.pool_state.trading_protocol_fee_bps()?;
    let to_protocol_fees_lst_amount = calc_swap_protocol_fees(CalcSwapProtocolFeesArgs {
        in_sol_value,
        out_sol_value,
        dst_lst_out,
        trading_protocol_fee_bps,
    })?;

    let total_dst_lst_out = dst_lst_out
        .checked_add(to_protocol_fees_lst_amount)
        .ok_or(SControllerError::MathError)?;
    if total_dst_lst_out > token_account_balance_program_agnostic(accounts.dst_pool_reserves)? {
        return Err(SControllerError::NotEnoughLiquidity.into());
    }

    transfer_tokens(
        TransferTokensAccounts {
            from: accounts.src_lst_acc,
            to: accounts.src_pool_reserves,
            token_program: accounts.src_lst_token_program,
            authority: accounts.signer,
        },
        amount,
    )?;
    transfer_tokens_signed(
        TransferTokensAccounts {
            from: accounts.dst_pool_reserves,
            to: accounts.protocol_fee_accumulator,
            token_program: accounts.dst_lst_token_program,
            authority: accounts.pool_state,
        },
        to_protocol_fees_lst_amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;
    transfer_tokens_signed(
        TransferTokensAccounts {
            from: accounts.dst_pool_reserves,
            to: accounts.dst_lst_acc,
            token_program: accounts.dst_lst_token_program,
            authority: accounts.pool_state,
        },
        dst_lst_out,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    sync_sol_value_unchecked(SrcLstPoolReservesOf(&accounts), src_lst_cpi, src_lst_index)?;
    sync_sol_value_unchecked(DstLstPoolReservesOf(&accounts), dst_lst_cpi, dst_lst_index)?;

    let end_total_sol_value = accounts.pool_state.total_sol_value()?;
    if end_total_sol_value < start_total_sol_value {
        return Err(SControllerError::PoolWouldLoseSolValue.into());
    }

    Ok(())
}

fn verify_swap_exact_in<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    SwapExactInIxArgs {
        src_lst_value_calc_accs,
        dst_lst_value_calc_accs,
        src_lst_index,
        dst_lst_index,
        min_amount_out,
        amount,
    }: SwapExactInIxArgs,
) -> Result<
    (
        SwapExactInAccounts<'a, 'info>,
        SwapExactInAmounts,
        SrcDstLstIndexes,
        SrcDstLstSolValueCalculatorCpis<'a, 'info>,
        PricingProgramPriceSwapCpi<'a, 'info>,
    ),
    ProgramError,
> {
    let src_lst_index = index_to_usize(src_lst_index)?;
    let dst_lst_index = index_to_usize(dst_lst_index)?;

    let actual: SwapExactInAccounts = load_accounts(accounts)?;

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
    let expected = free_args.resolve_exact_in()?;

    swap_exact_in_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    swap_exact_in_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

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
        .get(SWAP_EXACT_IN_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let (src_dst_cpis, pricing_cpi) = verify_swap_cpis(
        actual,
        accounts_suffix_slice,
        src_dst_lst_value_calc_accs,
        src_dst_lst_indexes,
    )?;

    Ok((
        actual,
        SwapExactInAmounts {
            min_amount_out,
            amount,
        },
        src_dst_lst_indexes,
        src_dst_cpis,
        pricing_cpi,
    ))
}
