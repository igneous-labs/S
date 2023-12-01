use s_controller_interface::{
    sync_sol_value_verify_account_keys, sync_sol_value_verify_account_privileges,
    SyncSolValueAccounts, SyncSolValueIxArgs, SYNC_SOL_VALUE_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    sync_sol_value_with_retval, try_lst_state_list_mut, try_pool_state, try_pool_state_mut,
    SyncSolValueFreeArgs,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_utils::token::token_account_balance;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    account_traits::{
        GetLstStateListAccountInfo, GetPoolReservesAccountInfo, GetPoolStateAccountInfo,
    },
    cpi::SolValueCalculatorCpi,
    verify::verify_not_rebalancing_and_not_disabled,
};

pub fn process_sync_sol_value(accounts: &[AccountInfo], args: SyncSolValueIxArgs) -> ProgramResult {
    let (accounts, cpi) = verify_sync_sol_value(accounts, &args)?;
    sync_sol_value_unchecked(&accounts, cpi, args.lst_index as usize)
}

/// SyncSolValue's full subroutine, exported for use by other instruction processors
pub fn sync_sol_value_unchecked<
    'a,
    'info,
    A: GetPoolReservesAccountInfo<'a, 'info>
        + GetPoolStateAccountInfo<'a, 'info>
        + GetLstStateListAccountInfo<'a, 'info>,
>(
    accounts: &A,
    cpi: SolValueCalculatorCpi<'a, 'info>,
    lst_index: usize,
) -> Result<(), ProgramError> {
    let lst_balance = token_account_balance(accounts.get_pool_reserves_account_info())?;
    let returned_sol_value = cpi.invoke_lst_to_sol(lst_balance)?;

    let mut pool_state_bytes = accounts
        .get_pool_state_account_info()
        .try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_bytes)?;

    let mut lst_state_list_bytes = accounts
        .get_lst_state_list_account_info()
        .try_borrow_mut_data()?;
    let lst_state_list = try_lst_state_list_mut(&mut lst_state_list_bytes)?;
    let lst_state = &mut lst_state_list[lst_index];

    sync_sol_value_with_retval(pool_state, lst_state, returned_sol_value)?;

    Ok(())
}

fn verify_sync_sol_value<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    SyncSolValueIxArgs { lst_index }: &SyncSolValueIxArgs,
) -> Result<
    (
        SyncSolValueAccounts<'a, 'info>,
        SolValueCalculatorCpi<'a, 'info>,
    ),
    ProgramError,
> {
    let actual = load_accounts(accounts)?;
    let actual = verify_sync_sol_value_accounts(actual, *lst_index)?;

    let accounts_suffix_slice = accounts
        .get(SYNC_SOL_VALUE_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let cpi = SolValueCalculatorCpi::from_ix_accounts(&actual, accounts_suffix_slice)?;
    cpi.verify_correct_sol_value_calculator_program(&actual, *lst_index)?;

    Ok((actual, cpi))
}

fn verify_sync_sol_value_accounts<'a, 'info, I: TryInto<usize>>(
    actual: SyncSolValueAccounts<'a, 'info>,
    lst_index: I,
) -> Result<SyncSolValueAccounts<'a, 'info>, ProgramError> {
    let free_args = SyncSolValueFreeArgs {
        lst_index,
        lst_state_list: actual.lst_state_list,
        lst_mint: actual.lst_mint,
    };
    let expected = free_args.resolve()?;

    sync_sol_value_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    sync_sol_value_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;

    verify_not_rebalancing_and_not_disabled(pool_state)?;

    Ok(actual)
}
