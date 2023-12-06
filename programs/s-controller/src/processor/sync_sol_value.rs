use s_controller_interface::{
    sync_sol_value_verify_account_keys, sync_sol_value_verify_account_privileges,
    SyncSolValueAccounts, SyncSolValueIxArgs, SYNC_SOL_VALUE_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    index_to_usize, sync_sol_value_with_retval, try_lst_state_list_mut, try_pool_state,
    try_pool_state_mut, SyncSolValueFreeArgs,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_utils::token::token_account_balance_program_agnostic;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    account_traits::{
        GetLstStateListAccountInfo, GetPoolReservesAccountInfo, GetPoolStateAccountInfo,
    },
    cpi::SolValueCalculatorCpi,
    verify::{verify_lst_sol_val_calc_cpi, verify_not_rebalancing_and_not_disabled},
};

pub fn process_sync_sol_value(accounts: &[AccountInfo], args: SyncSolValueIxArgs) -> ProgramResult {
    let (accounts, cpi) = verify_sync_sol_value(accounts, &args)?;
    let lst_index: usize = args.lst_index.try_into().unwrap(); // lst_index checked in verify
    sync_sol_value_unchecked(
        SyncSolValueUncheckedAccounts::from(accounts),
        cpi,
        lst_index,
    )
}

#[derive(Clone, Copy, Debug)]
pub struct SyncSolValueUncheckedAccounts<'me, 'info> {
    pub pool_reserves: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub lst_state_list: &'me AccountInfo<'info>,
}

impl<'me, 'info, A> From<A> for SyncSolValueUncheckedAccounts<'me, 'info>
where
    A: GetPoolReservesAccountInfo<'me, 'info>
        + GetPoolStateAccountInfo<'me, 'info>
        + GetLstStateListAccountInfo<'me, 'info>,
{
    fn from(ix_accounts: A) -> Self {
        Self {
            pool_reserves: ix_accounts.get_pool_reserves_account_info(),
            pool_state: ix_accounts.get_pool_state_account_info(),
            lst_state_list: ix_accounts.get_lst_state_list_account_info(),
        }
    }
}

/// SyncSolValue's full subroutine, exported for use by other instruction processors
pub fn sync_sol_value_unchecked<'a, 'info>(
    SyncSolValueUncheckedAccounts {
        pool_reserves,
        pool_state,
        lst_state_list,
    }: SyncSolValueUncheckedAccounts<'a, 'info>,
    cpi: SolValueCalculatorCpi<'a, 'info>,
    lst_index: usize,
) -> Result<(), ProgramError> {
    let lst_balance = token_account_balance_program_agnostic(pool_reserves)?;
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
    let lst_index = index_to_usize(*lst_index)?;
    let actual = verify_sync_sol_value_base_accounts(actual, lst_index)?;

    let accounts_suffix_slice = accounts
        .get(SYNC_SOL_VALUE_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let cpi = verify_lst_sol_val_calc_cpi(actual, accounts_suffix_slice, lst_index)?;
    Ok((actual, cpi))
}

fn verify_sync_sol_value_base_accounts<'a, 'info>(
    actual: SyncSolValueAccounts<'a, 'info>,
    lst_index: usize,
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
