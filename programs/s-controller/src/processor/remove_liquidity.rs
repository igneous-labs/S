use s_controller_interface::{
    remove_liquidity_verify_account_keys, remove_liquidity_verify_account_privileges,
    RemoveLiquidityAccounts, RemoveLiquidityIxArgs, SControllerError,
    REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    calc_lp_tokens_sol_value, calc_remove_liquidity_protocol_fees, index_to_usize,
    program::{POOL_STATE_BUMP, POOL_STATE_SEED},
    try_pool_state, CalcRemoveLiquidityProtocolFeesArgs, LpTokenRateArgs, PoolStateAccount,
    RemoveLiquidityFreeArgs, RemoveLiquidityIxFullArgs,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_token_lib::{
    burn_invoke, mint_supply, transfer_checked_decimal_agnostic_invoke_signed, BurnAccounts,
    TransferCheckedAccounts,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    cpi::{PricingProgramIxArgs, PricingProgramPriceLpCpi, SolValueCalculatorCpi},
    verify::{verify_lp_cpis, verify_not_rebalancing_and_not_disabled, VerifyLpCpiAccounts},
};

use super::{sync_sol_value_unchecked, SyncSolValueUncheckedAccounts};

pub fn process_remove_liquidity(
    accounts: &[AccountInfo],
    args: RemoveLiquidityIxArgs,
) -> ProgramResult {
    let (
        accounts,
        RemoveLiquidityIxFullArgs {
            lst_index,
            lp_token_amount,
        },
        lst_cpi,
        pricing_cpi,
    ) = verify_remove_liquidity(accounts, args)?;

    let sync_sol_value_accounts = SyncSolValueUncheckedAccounts::from(accounts);
    sync_sol_value_unchecked(sync_sol_value_accounts, lst_cpi, lst_index)?;

    let pool_total_sol_value = accounts.pool_state.total_sol_value()?;
    let lp_token_supply = mint_supply(accounts.lp_token_mint)?;
    let lp_tokens_sol_value = calc_lp_tokens_sol_value(
        LpTokenRateArgs {
            lp_token_supply,
            pool_total_sol_value,
        },
        lp_token_amount,
    )?;

    let lp_tokens_sol_value_after_fees =
        pricing_cpi.invoke_price_lp_tokens_to_redeem(PricingProgramIxArgs {
            amount: lp_token_amount,
            sol_value: lp_tokens_sol_value,
        })?;
    if lp_tokens_sol_value_after_fees > lp_tokens_sol_value {
        return Err(SControllerError::PoolWouldLoseSolValue.into());
    }

    let to_user_lst_amount = lst_cpi
        .invoke_sol_to_lst(lp_tokens_sol_value_after_fees)?
        .min;
    let to_protocol_fees_lst_amount =
        calc_remove_liquidity_protocol_fees(CalcRemoveLiquidityProtocolFeesArgs {
            lp_tokens_sol_value,
            lp_tokens_sol_value_after_fees,
            to_user_lst_amount,
            lp_protocol_fee_bps: accounts.pool_state.lp_protocol_fee_bps()?,
        })?;

    if to_user_lst_amount == 0 {
        return Err(SControllerError::ZeroValue.into());
    }

    burn_invoke(
        BurnAccounts {
            mint: accounts.lp_token_mint,
            burn_from: accounts.src_lp_acc,
            burn_from_authority: accounts.signer,
            token_program: accounts.lp_token_program,
        },
        lp_token_amount,
    )?;

    transfer_checked_decimal_agnostic_invoke_signed(
        TransferCheckedAccounts {
            to: accounts.dst_lst_acc,
            token_program: accounts.lst_token_program,
            from: accounts.pool_reserves,
            authority: accounts.pool_state,
            mint: accounts.lst_mint,
        },
        to_user_lst_amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    transfer_checked_decimal_agnostic_invoke_signed(
        TransferCheckedAccounts {
            to: accounts.protocol_fee_accumulator,
            token_program: accounts.lst_token_program,
            from: accounts.pool_reserves,
            authority: accounts.pool_state,
            mint: accounts.lst_mint,
        },
        to_protocol_fees_lst_amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    sync_sol_value_unchecked(sync_sol_value_accounts, lst_cpi, lst_index)
}

fn verify_remove_liquidity<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    RemoveLiquidityIxArgs {
        lst_value_calc_accs,
        lst_index,
        lp_token_amount,
    }: RemoveLiquidityIxArgs,
) -> Result<
    (
        RemoveLiquidityAccounts<'a, 'info>,
        RemoveLiquidityIxFullArgs,
        SolValueCalculatorCpi<'a, 'info>,
        PricingProgramPriceLpCpi<'a, 'info>,
    ),
    ProgramError,
> {
    if lp_token_amount == 0 {
        return Err(SControllerError::ZeroValue.into());
    }

    let lst_index = index_to_usize(lst_index)?;

    let actual: RemoveLiquidityAccounts = load_accounts(accounts)?;

    let free_args = RemoveLiquidityFreeArgs {
        lst_index,
        signer: *actual.signer.key,
        dst_lst_acc: *actual.dst_lst_acc.key,
        src_lp_acc: *actual.src_lp_acc.key,
        pool_state: actual.pool_state,
        lst_state_list: actual.lst_state_list,
        lst_mint: actual.lst_mint,
    };
    let expected = free_args.resolve()?;

    remove_liquidity_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    remove_liquidity_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    let accounts_suffix_slice = accounts
        .get(REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let (lst_cpi, pricing_cpi) = verify_lp_cpis(
        VerifyLpCpiAccounts::from(actual),
        accounts_suffix_slice,
        lst_value_calc_accs,
        lst_index,
    )?;

    Ok((
        actual,
        RemoveLiquidityIxFullArgs {
            lst_index,
            lp_token_amount,
        },
        lst_cpi,
        pricing_cpi,
    ))
}
