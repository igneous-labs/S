use s_controller_interface::{
    set_sol_value_calculator_verify_account_keys,
    set_sol_value_calculator_verify_account_privileges, SetSolValueCalculatorAccounts,
    SetSolValueCalculatorIxArgs, SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    index_to_usize, try_lst_state_list_mut, try_pool_state, SetSolValueCalculatorFreeArgs,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{cpi::SolValueCalculatorCpi, verify::verify_not_rebalancing_and_not_disabled};

use super::{sync_sol_value_unchecked, SyncSolValueUncheckedAccounts};

pub fn process_set_sol_value_calculator(
    accounts: &[AccountInfo],
    args: SetSolValueCalculatorIxArgs,
) -> ProgramResult {
    let (accounts, lst_index, cpi) = verify_set_sol_value_calculator(accounts, args)?;

    {
        let mut lst_state_list_data = accounts.lst_state_list.try_borrow_mut_data()?;
        let lst_state_list = try_lst_state_list_mut(&mut lst_state_list_data)?;
        // lst_index checked in verify
        lst_state_list[lst_index].sol_value_calculator = *cpi.program.key;
    }

    sync_sol_value_unchecked(
        SyncSolValueUncheckedAccounts::from(accounts),
        cpi,
        lst_index,
    )
}

fn verify_set_sol_value_calculator<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    SetSolValueCalculatorIxArgs { lst_index }: SetSolValueCalculatorIxArgs,
) -> Result<
    (
        SetSolValueCalculatorAccounts<'a, 'info>,
        usize,
        SolValueCalculatorCpi<'a, 'info>,
    ),
    ProgramError,
> {
    let lst_index = index_to_usize(lst_index)?;
    let actual: SetSolValueCalculatorAccounts = load_accounts(accounts)?;

    let free_args = SetSolValueCalculatorFreeArgs {
        lst_index,
        pool_state: actual.pool_state,
        lst_state_list: actual.lst_state_list,
        lst_mint: actual.lst_mint,
    };
    let expected = free_args.resolve()?;

    set_sol_value_calculator_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    set_sol_value_calculator_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    let accounts_suffix_slice = accounts
        .get(SET_SOL_VALUE_CALCULATOR_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let cpi = SolValueCalculatorCpi::from_lst_mint_and_account_suffix_slice(
        actual.lst_mint,
        accounts_suffix_slice,
    )?;

    Ok((actual, lst_index, cpi))
}
