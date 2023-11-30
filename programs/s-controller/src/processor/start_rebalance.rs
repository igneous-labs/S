use s_controller_interface::{
    start_rebalance_verify_account_keys, start_rebalance_verify_account_privileges,
    SControllerError, StartRebalanceAccounts, StartRebalanceIxArgs,
    START_REBALANCE_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{try_lst_state_list, try_pool_state, StartRebalanceFreeArgs};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    account_traits::{DstLstMintOf, SrcLstMintOf},
    cpi::{SolValueCalculatorCpi, SrcDstLstSolValueCalculatorCpis},
    verify::{verify_lst_input_not_disabled, verify_not_rebalancing_and_not_disabled},
};

pub fn process_start_rebalance(
    accounts: &[AccountInfo],
    args: StartRebalanceIxArgs,
) -> ProgramResult {
    let _todo = verify_start_rebalance(accounts, &args)?;
    Ok(())
}

fn verify_start_rebalance<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    StartRebalanceIxArgs {
        src_lst_calc_accs,
        src_lst_index,
        dst_lst_index,
        ..
    }: &StartRebalanceIxArgs,
) -> Result<
    (
        StartRebalanceAccounts<'a, 'info>,
        SrcDstLstSolValueCalculatorCpis<'a, 'info>,
    ),
    ProgramError,
> {
    let actual: StartRebalanceAccounts = load_accounts(accounts)?;

    let free_args = StartRebalanceFreeArgs {
        payer: *actual.payer.key,
        withdraw_to: *actual.withdraw_to.key,
        src_lst_index: *src_lst_index,
        dst_lst_index: *dst_lst_index,
        lst_state_list: actual.lst_state_list,
        pool_state: actual.pool_state,
        src_lst_mint: actual.src_lst_mint,
        dst_lst_mint: actual.dst_lst_mint,
    };
    let expected = free_args.resolve()?;

    start_rebalance_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    start_rebalance_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    let lst_state_list_bytes = actual.lst_state_list.try_borrow_data()?;
    let lst_state_list = try_lst_state_list(&lst_state_list_bytes)?;
    let dst_lst_state = lst_state_list[*dst_lst_index as usize]; // dst_lst_index checked above
    verify_lst_input_not_disabled(&dst_lst_state)?;

    let src_lst_accounts_suffix_end = START_REBALANCE_IX_ACCOUNTS_LEN
        .checked_add((*src_lst_calc_accs).into())
        .ok_or(SControllerError::MathError)?;
    let src_lst_accounts_suffix_slice = accounts
        .get(START_REBALANCE_IX_ACCOUNTS_LEN..src_lst_accounts_suffix_end)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let src_lst_cpi = SolValueCalculatorCpi::from_ix_accounts(
        &SrcLstMintOf(&actual),
        src_lst_accounts_suffix_slice,
    )?;
    src_lst_cpi.verify_correct_sol_value_calculator_program(&actual, *src_lst_index)?;

    let dst_lst_accounts_suffix_slice = accounts
        .get(src_lst_accounts_suffix_end..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let dst_lst_cpi = SolValueCalculatorCpi::from_ix_accounts(
        &DstLstMintOf(&actual),
        dst_lst_accounts_suffix_slice,
    )?;
    dst_lst_cpi.verify_correct_sol_value_calculator_program(&actual, *dst_lst_index)?;

    // TODO: check ix sysvar

    Ok((
        actual,
        SrcDstLstSolValueCalculatorCpis {
            src_lst: src_lst_cpi,
            dst_lst: dst_lst_cpi,
        },
    ))
}
