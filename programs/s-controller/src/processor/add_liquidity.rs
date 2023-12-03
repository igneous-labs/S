use s_controller_interface::{
    add_liquidity_verify_account_keys, add_liquidity_verify_account_privileges,
    AddLiquidityAccounts, AddLiquidityIxArgs, SControllerError, ADD_LIQUIDITY_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    calc_lp_tokens_to_mint,
    program::{POOL_STATE_BUMP, POOL_STATE_SEED},
    try_lst_state_list, try_pool_state, AddLiquidityFreeArgs, LpTokenRateArgs,
};
use sanctum_onchain_utils::{
    token_2022::{mint_to_signed, MintToAccounts},
    token_program::{tranfer_tokens, TransferAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use sanctum_utils::token::mint_supply;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    cpi::{PricingProgramIxArgs, PricingProgramLiquidityCpi, SolValueCalculatorCpi},
    verify::{verify_lst_input_not_disabled, verify_not_rebalancing_and_not_disabled},
};

use super::sync_sol_value_unchecked;

pub fn process_add_liquidity(accounts: &[AccountInfo], args: AddLiquidityIxArgs) -> ProgramResult {
    let (
        accounts,
        AddLiquidityIxArgs {
            lst_index, amount, ..
        },
        lst_cpi,
        pricing_cpi,
    ) = verify_add_liquidity(accounts, args)?;
    // lst_index checked in verify
    let lst_index: usize = lst_index.try_into().unwrap();
    sync_sol_value_unchecked(&accounts, lst_cpi, lst_index)?;

    let sol_value_to_add = lst_cpi.invoke_lst_to_sol(amount)?;
    let final_sol_value_to_add =
        pricing_cpi.invoke_price_lp_tokens_to_mint(PricingProgramIxArgs {
            amount,
            sol_value: sol_value_to_add,
        })?;

    let pool_total_sol_value = {
        let pool_state_bytes = accounts.pool_state.try_borrow_data()?;
        let pool_state = try_pool_state(&pool_state_bytes)?;
        pool_state.total_sol_value
    };
    let lp_token_supply = mint_supply(accounts.lp_token_mint)?;
    let lp_tokens_to_mint = calc_lp_tokens_to_mint(
        LpTokenRateArgs {
            lp_token_supply,
            pool_total_sol_value,
        },
        final_sol_value_to_add,
    )?;

    tranfer_tokens(
        TransferAccounts {
            from: accounts.src_lst_acc,
            to: accounts.pool_reserves,
            token_program: accounts.lst_token_program,
            authority: accounts.signer,
        },
        amount,
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
    sync_sol_value_unchecked(&accounts, lst_cpi, lst_index)
}

fn verify_add_liquidity<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    args: AddLiquidityIxArgs,
) -> Result<
    (
        AddLiquidityAccounts<'a, 'info>,
        AddLiquidityIxArgs,
        SolValueCalculatorCpi<'a, 'info>,
        PricingProgramLiquidityCpi<'a, 'info>,
    ),
    ProgramError,
> {
    let actual: AddLiquidityAccounts = load_accounts(accounts)?;

    let free_args = AddLiquidityFreeArgs {
        lst_index: args.lst_index,
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
    let lst_index: usize = args.lst_index.try_into().unwrap();
    let dst_lst_state = lst_state_list[lst_index];
    verify_lst_input_not_disabled(&dst_lst_state)?;

    let lst_value_calc_suffix_end = ADD_LIQUIDITY_IX_ACCOUNTS_LEN
        .checked_add((args.lst_value_calc_accs).into())
        .ok_or(SControllerError::MathError)?;
    let lst_accounts_suffix_slice = accounts
        .get(ADD_LIQUIDITY_IX_ACCOUNTS_LEN..lst_value_calc_suffix_end)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let lst_cpi = SolValueCalculatorCpi::from_ix_accounts(&actual, lst_accounts_suffix_slice)?;
    lst_cpi.verify_correct_sol_value_calculator_program(&actual, args.lst_index)?;

    let pricing_accounts_suffix_slice = accounts
        .get(lst_value_calc_suffix_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let pricing_cpi =
        PricingProgramLiquidityCpi::from_ix_accounts(&actual, pricing_accounts_suffix_slice)?;
    pricing_cpi.verify_correct_pricing_program(&actual)?;

    Ok((actual, args, lst_cpi, pricing_cpi))
}
