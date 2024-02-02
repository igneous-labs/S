use s_controller_interface::{
    end_rebalance_verify_account_keys, end_rebalance_verify_account_privileges,
    EndRebalanceAccounts, PoolState, RebalanceRecord, SControllerError,
    END_REBALANCE_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    try_pool_state, try_pool_state_mut, try_rebalance_record, EndRebalanceFreeArgs,
    PoolStateAccount, U8Bool, U8BoolMut,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_system_program_lib::{close_account, CloseAccountAccounts};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    cpi::SolValueCalculatorCpi,
    verify::{verify_lst_sol_val_calc_cpi, VerifyLstSolValCalcCpiAccounts},
};

use super::{sync_sol_value_unchecked, SyncSolValueUncheckedAccounts};

pub fn process_end_rebalance(accounts: &[AccountInfo]) -> ProgramResult {
    let (accounts, cpi, dst_lst_index) = verify_end_rebalance(accounts)?;
    // braces to limit scope of pool_state_data borrow
    {
        let mut pool_state_data = accounts.pool_state.try_borrow_mut_data()?;
        let pool_state = try_pool_state_mut(&mut pool_state_data)?;
        U8BoolMut(&mut pool_state.is_rebalancing).set_false();
    }
    let old_total_sol_value = {
        let rebalance_record_data = accounts.rebalance_record.try_borrow_data()?;
        let RebalanceRecord {
            old_total_sol_value,
            ..
        } = try_rebalance_record(&rebalance_record_data)?;
        *old_total_sol_value
    };

    sync_sol_value_unchecked(
        SyncSolValueUncheckedAccounts::from(accounts),
        cpi,
        dst_lst_index,
    )?;

    if accounts.pool_state.total_sol_value()? < old_total_sol_value {
        return Err(SControllerError::PoolWouldLoseSolValue.into());
    }

    close_account(CloseAccountAccounts {
        refund_rent_to: accounts.pool_state,
        close: accounts.rebalance_record,
    })
}

fn verify_end_rebalance<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
) -> Result<
    (
        EndRebalanceAccounts<'a, 'info>,
        SolValueCalculatorCpi<'a, 'info>,
        usize,
    ),
    ProgramError,
> {
    let actual: EndRebalanceAccounts = load_accounts(accounts)?;

    let free_args = EndRebalanceFreeArgs {
        pool_state: actual.pool_state,
        lst_state_list: actual.lst_state_list,
        rebalance_record: actual.rebalance_record,
        dst_lst_mint: actual.dst_lst_mint,
    };
    let (expected, dst_lst_index) = free_args.resolve()?;

    end_rebalance_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    end_rebalance_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_is_rebalancing(pool_state)?;

    let accounts_suffix_slice = accounts
        .get(END_REBALANCE_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let dst_lst_cpi = verify_lst_sol_val_calc_cpi(
        VerifyLstSolValCalcCpiAccounts::from(actual),
        accounts_suffix_slice,
        dst_lst_index,
    )?;

    Ok((actual, dst_lst_cpi, dst_lst_index))
}

const fn verify_is_rebalancing(pool_state: &PoolState) -> Result<(), SControllerError> {
    if U8Bool(pool_state.is_rebalancing).is_true() {
        Ok(())
    } else {
        Err(SControllerError::PoolNotRebalancing)
    }
}
