use s_controller_interface::{
    remove_liquidity_verify_account_keys, remove_liquidity_verify_account_privileges,
    RemoveLiquidityAccounts, RemoveLiquidityIxArgs, SControllerError,
    REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    calc_lp_tokens_sol_value, calc_remove_liquidity_protocol_fees,
    program::{POOL_STATE_BUMP, POOL_STATE_SEED},
    try_pool_state, CalcRemoveLiquidityProtocolFeesArgs, LpTokenRateArgs, PoolStateAccount,
    RemoveLiquidityFreeArgs, RemoveLiquidityIxFullArgs,
};
use sanctum_onchain_utils::{
    token_2022::{burn_tokens, BurnTokensAccounts},
    token_program::{transfer_tokens_signed, TransferTokensAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use sanctum_utils::token::token_2022_mint_supply;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    cpi::{PricingProgramIxArgs, PricingProgramPriceLpCpi, SolValueCalculatorCpi},
    verify::verify_not_rebalancing_and_not_disabled,
};

use super::sync_sol_value_unchecked;

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

    // lst_index checked in verify
    let lst_index: usize = lst_index.try_into().unwrap();
    sync_sol_value_unchecked(accounts, lst_cpi, lst_index)?;

    let pool_total_sol_value = accounts.pool_state.total_sol_value()?;
    let lp_token_supply = token_2022_mint_supply(accounts.lp_token_mint)?;
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

    let to_user_lst_amount = lst_cpi.invoke_sol_to_lst(lp_tokens_sol_value_after_fees)?;
    let to_protocol_fees_lst_amount =
        calc_remove_liquidity_protocol_fees(CalcRemoveLiquidityProtocolFeesArgs {
            lp_tokens_sol_value,
            lp_tokens_sol_value_after_fees,
            to_user_lst_amount,
            lp_protocol_fee_bps: accounts.pool_state.lp_protocol_fee_bps()?,
        })?;

    burn_tokens(
        BurnTokensAccounts {
            mint: accounts.lp_token_mint,
            burn_from: accounts.src_lp_acc,
            burn_from_authority: accounts.signer,
        },
        lp_token_amount,
    )?;

    transfer_tokens_signed(
        TransferTokensAccounts {
            to: accounts.dst_lst_acc,
            token_program: accounts.lst_token_program,
            from: accounts.pool_reserves,
            authority: accounts.pool_state,
        },
        to_user_lst_amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    transfer_tokens_signed(
        TransferTokensAccounts {
            to: accounts.protocol_fee_accumulator,
            token_program: accounts.lst_token_program,
            from: accounts.pool_reserves,
            authority: accounts.pool_state,
        },
        to_protocol_fees_lst_amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    sync_sol_value_unchecked(accounts, lst_cpi, lst_index)
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

    remove_liquidity_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    remove_liquidity_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    let lst_value_calc_suffix_end = REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN
        .checked_add(lst_value_calc_accs.into())
        .ok_or(SControllerError::MathError)?;
    let lst_accounts_suffix_slice = accounts
        .get(REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN..lst_value_calc_suffix_end)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let lst_cpi = SolValueCalculatorCpi::from_ix_accounts(actual, lst_accounts_suffix_slice)?;
    lst_cpi.verify_correct_sol_value_calculator_program(actual, lst_index)?;

    let pricing_accounts_suffix_slice = accounts
        .get(lst_value_calc_suffix_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let pricing_cpi =
        PricingProgramPriceLpCpi::from_ix_accounts(actual, pricing_accounts_suffix_slice)?;
    pricing_cpi.verify_correct_pricing_program(actual)?;

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
