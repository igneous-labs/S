use s_controller_interface::{
    add_liquidity_verify_account_keys, add_liquidity_verify_account_privileges,
    AddLiquidityAccounts, AddLiquidityIxArgs, SControllerError, ADD_LIQUIDITY_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    calc_add_liquidity, calc_lp_tokens_to_mint, index_to_usize,
    program::{POOL_STATE_BUMP, POOL_STATE_SEED},
    try_lst_state_list, try_pool_state, AddLiquidityFreeArgs, AddLiquidityIxFullArgs,
    CalcAddLiquidityArgs, CalcAddLiquidityResult, LpTokenRateArgs, PoolStateAccount,
};
use sanctum_onchain_utils::{
    token_2022::{mint_to_signed, MintToAccounts},
    token_program::{transfer_tokens, TransferTokensAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use sanctum_utils::token::token_2022_mint_supply;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    cpi::{PricingProgramIxArgs, PricingProgramPriceLpCpi, SolValueCalculatorCpi},
    verify::{
        verify_lp_cpis, verify_lst_input_not_disabled, verify_not_rebalancing_and_not_disabled,
        VerifyLpCpiAccounts,
    },
};

use super::{sync_sol_value_unchecked, SyncSolValueUncheckedAccounts};

pub fn process_add_liquidity(accounts: &[AccountInfo], args: AddLiquidityIxArgs) -> ProgramResult {
    let (
        accounts,
        AddLiquidityIxFullArgs {
            lst_index,
            lst_amount,
        },
        lst_cpi,
        pricing_cpi,
    ) = verify_add_liquidity(accounts, args)?;

    let sync_sol_value_accounts = SyncSolValueUncheckedAccounts::from(accounts);

    sync_sol_value_unchecked(sync_sol_value_accounts, lst_cpi, lst_index)?;

    let start_total_sol_value = accounts.pool_state.total_sol_value()?;

    let lst_amount_sol_value = lst_cpi.invoke_lst_to_sol(lst_amount)?;
    let lst_amount_sol_value_after_fees =
        pricing_cpi.invoke_price_lp_tokens_to_mint(PricingProgramIxArgs {
            amount: lst_amount,
            sol_value: lst_amount_sol_value,
        })?;

    let CalcAddLiquidityResult {
        to_reserves_lst_amount,
        to_protocol_fees_lst_amount,
    } = calc_add_liquidity(CalcAddLiquidityArgs {
        lst_amount,
        lst_amount_sol_value,
        lst_amount_sol_value_after_fees,
        lp_protocol_fee_bps: accounts.pool_state.lp_protocol_fee_bps()?,
    })?;

    let pool_total_sol_value = accounts.pool_state.total_sol_value()?;
    let lp_token_supply = token_2022_mint_supply(accounts.lp_token_mint)?;
    let lp_tokens_to_mint = calc_lp_tokens_to_mint(
        LpTokenRateArgs {
            lp_token_supply,
            pool_total_sol_value,
        },
        lst_amount_sol_value_after_fees,
    )?;

    transfer_tokens(
        TransferTokensAccounts {
            from: accounts.src_lst_acc,
            to: accounts.pool_reserves,
            token_program: accounts.lst_token_program,
            authority: accounts.signer,
        },
        to_reserves_lst_amount,
    )?;
    transfer_tokens(
        TransferTokensAccounts {
            from: accounts.src_lst_acc,
            to: accounts.protocol_fee_accumulator,
            token_program: accounts.lst_token_program,
            authority: accounts.signer,
        },
        to_protocol_fees_lst_amount,
    )?;
    mint_to_signed(
        MintToAccounts {
            mint: accounts.lp_token_mint,
            mint_to: accounts.dst_lp_acc,
            mint_authority: accounts.pool_state,
        },
        lp_tokens_to_mint,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;
    sync_sol_value_unchecked(sync_sol_value_accounts, lst_cpi, lst_index)?;

    let end_total_sol_value = accounts.pool_state.total_sol_value()?;
    if end_total_sol_value < start_total_sol_value {
        return Err(SControllerError::PoolWouldLoseSolValue.into());
    }

    Ok(())
}

fn verify_add_liquidity<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    AddLiquidityIxArgs {
        lst_value_calc_accs,
        lst_index,
        lst_amount,
    }: AddLiquidityIxArgs,
) -> Result<
    (
        AddLiquidityAccounts<'a, 'info>,
        AddLiquidityIxFullArgs,
        SolValueCalculatorCpi<'a, 'info>,
        PricingProgramPriceLpCpi<'a, 'info>,
    ),
    ProgramError,
> {
    let lst_index = index_to_usize(lst_index)?;

    let actual: AddLiquidityAccounts = load_accounts(accounts)?;

    let free_args = AddLiquidityFreeArgs {
        lst_index,
        signer: *actual.signer.key,
        src_lst_acc: *actual.src_lst_acc.key,
        dst_lp_acc: *actual.dst_lp_acc.key,
        pool_state: actual.pool_state,
        lst_state_list: actual.lst_state_list,
        lst_mint: actual.lst_mint,
    };
    let expected = free_args.resolve()?;

    add_liquidity_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    add_liquidity_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    let lst_state_list_bytes = actual.lst_state_list.try_borrow_data()?;
    let lst_state_list = try_lst_state_list(&lst_state_list_bytes)?;
    // dst_lst_index checked above
    let dst_lst_state = lst_state_list[lst_index];
    verify_lst_input_not_disabled(&dst_lst_state)?;

    let accounts_suffix_slice = accounts
        .get(ADD_LIQUIDITY_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let (lst_cpi, pricing_cpi) = verify_lp_cpis(
        VerifyLpCpiAccounts::from(actual),
        accounts_suffix_slice,
        lst_value_calc_accs,
        lst_index,
    )?;

    Ok((
        actual,
        AddLiquidityIxFullArgs {
            lst_index,
            lst_amount,
        },
        lst_cpi,
        pricing_cpi,
    ))
}
