use s_controller_interface::{
    SyncSolValueAccounts, SyncSolValueIxArgs, SYNC_SOL_VALUE_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{sync_sol_value_with_retval, try_lst_state_list_mut, try_pool_state_mut};
use sanctum_onchain_utils::utils::load_accounts;
use sanctum_utils::token::token_account_balance;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{cpi::SolValueCalculatorCpi, verify::verify_sync_sol_value_accounts};

pub fn process_sync_sol_value(accounts: &[AccountInfo], args: SyncSolValueIxArgs) -> ProgramResult {
    let (accounts, cpi) = verify_sync_sol_value(accounts, &args)?;
    sync_sol_value_unchecked(accounts, cpi, args.lst_index as usize)?;
    Ok(())
}

/// SyncSolValue's full subroutine, exported for use by other instruction processors
pub fn sync_sol_value_unchecked<'a, 'info>(
    SyncSolValueAccounts {
        pool_state,
        lst_state_list,
        pool_reserves,
        ..
    }: SyncSolValueAccounts<'a, 'info>,
    cpi: SolValueCalculatorCpi<'a, 'info>,
    lst_index: usize,
) -> Result<(), ProgramError> {
    let lst_balance = token_account_balance(pool_reserves)?;
    let returned_sol_value = cpi.invoke_lst_to_sol(lst_balance)?;

    let mut pool_state_bytes = pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_bytes)?;

    let mut lst_state_list_bytes = lst_state_list.try_borrow_mut_data()?;
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

    let cpi = SolValueCalculatorCpi::from_accounts(&actual, accounts_suffix_slice)?;
    cpi.verify_correct_sol_value_calculator_program(&actual, *lst_index)?;

    Ok((actual, cpi))
}
