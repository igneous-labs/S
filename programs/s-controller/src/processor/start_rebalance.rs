use s_controller_interface::{
    start_rebalance_verify_account_keys, start_rebalance_verify_account_privileges,
    SControllerError, StartRebalanceAccounts, StartRebalanceIxArgs, END_REBALANCE_IX_DISCM,
    START_REBALANCE_IX_ACCOUNTS_LEN,
};
use s_controller_lib::{
    index_to_usize,
    program::{POOL_STATE_BUMP, POOL_STATE_SEED, REBALANCE_RECORD_BUMP, REBALANCE_RECORD_SEED},
    try_lst_state_list, try_pool_state, try_pool_state_mut, try_rebalance_record_mut,
    PoolStateAccount, SrcDstLstIndexes, StartRebalanceFreeArgs, U8BoolMut, REBALANCE_RECORD_SIZE,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_system_program_lib::{
    allocate_invoke_signed, assign_invoke_signed, space_to_u64, transfer_direct_increment,
    TransferAccounts,
};
use sanctum_token_lib::{
    token_account_balance, transfer_checked_decimal_agnostic_invoke_signed, TransferCheckedAccounts,
};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::instructions::{load_current_index_checked, load_instruction_at_checked},
};

use crate::{
    account_traits::{DstLstPoolReservesOf, SrcLstPoolReservesOf},
    cpi::SrcDstLstSolValueCalculatorCpis,
    verify::{
        verify_lst_input_not_disabled, verify_not_rebalancing_and_not_disabled,
        verify_src_dst_lst_sol_val_calc_cpis, VerifySrcDstLstSolValCalcCpiAccounts,
    },
};

use super::{sync_sol_value_unchecked, SyncSolValueUncheckedAccounts};

pub fn process_start_rebalance(
    accounts: &[AccountInfo],
    args: StartRebalanceIxArgs,
) -> ProgramResult {
    let (
        accounts,
        SrcDstLstSolValueCalculatorCpis {
            src_lst: src_lst_cpi,
            dst_lst: dst_lst_cpi,
        },
        SrcDstLstIndexes {
            src_lst_index,
            dst_lst_index,
        },
    ) = verify_start_rebalance(accounts, &args)?;

    let src_sync_sol_value_accounts =
        SyncSolValueUncheckedAccounts::from(SrcLstPoolReservesOf(accounts));
    sync_sol_value_unchecked(src_sync_sol_value_accounts, src_lst_cpi, src_lst_index)?;
    sync_sol_value_unchecked(
        SyncSolValueUncheckedAccounts::from(DstLstPoolReservesOf(accounts)),
        dst_lst_cpi,
        dst_lst_index,
    )?;

    let old_total_sol_value = accounts.pool_state.total_sol_value()?;

    transfer_checked_decimal_agnostic_invoke_signed(
        TransferCheckedAccounts {
            token_program: accounts.src_lst_token_program,
            from: accounts.src_pool_reserves,
            to: accounts.withdraw_to,
            authority: accounts.pool_state,
            mint: accounts.src_lst_mint,
        },
        args.amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    sync_sol_value_unchecked(src_sync_sol_value_accounts, src_lst_cpi, src_lst_index)?;

    allocate_invoke_signed(
        accounts.rebalance_record,
        space_to_u64(REBALANCE_RECORD_SIZE)?,
        &[&[REBALANCE_RECORD_SEED, &[REBALANCE_RECORD_BUMP]]],
    )?;
    assign_invoke_signed(
        accounts.rebalance_record,
        s_controller_lib::program::ID,
        &[&[REBALANCE_RECORD_SEED, &[REBALANCE_RECORD_BUMP]]],
    )?;
    transfer_direct_increment(
        TransferAccounts {
            from: accounts.pool_state,
            to: accounts.rebalance_record,
        },
        1,
    )?;

    let mut rebalance_record_data = accounts.rebalance_record.try_borrow_mut_data()?;
    let rebalance_record = try_rebalance_record_mut(&mut rebalance_record_data)?;
    rebalance_record.dst_lst_index = args.dst_lst_index;
    rebalance_record.old_total_sol_value = old_total_sol_value;

    let mut pool_state_data = accounts.pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_data)?;
    U8BoolMut(&mut pool_state.is_rebalancing).set_true();

    Ok(())
}

fn verify_start_rebalance<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    StartRebalanceIxArgs {
        src_lst_calc_accs,
        src_lst_index,
        dst_lst_index,
        amount: _,
        min_starting_src_lst,
        max_starting_dst_lst,
    }: &StartRebalanceIxArgs,
) -> Result<
    (
        StartRebalanceAccounts<'a, 'info>,
        SrcDstLstSolValueCalculatorCpis<'a, 'info>,
        SrcDstLstIndexes,
    ),
    ProgramError,
> {
    let src_lst_index = index_to_usize(*src_lst_index)?;
    let dst_lst_index = index_to_usize(*dst_lst_index)?;

    let actual: StartRebalanceAccounts = load_accounts(accounts)?;

    let free_args = StartRebalanceFreeArgs {
        withdraw_to: *actual.withdraw_to.key,
        src_lst_index,
        dst_lst_index,
        lst_state_list: actual.lst_state_list,
        pool_state: actual.pool_state,
        src_lst_mint: actual.src_lst_mint,
        dst_lst_mint: actual.dst_lst_mint,
    };
    let expected = free_args.resolve()?;

    start_rebalance_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    start_rebalance_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    if token_account_balance(actual.src_pool_reserves)? < *min_starting_src_lst {
        return Err(SControllerError::SlippageToleranceExceeded.into());
    }
    if token_account_balance(actual.dst_pool_reserves)? > *max_starting_dst_lst {
        return Err(SControllerError::SlippageToleranceExceeded.into());
    }

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    let lst_state_list_bytes = actual.lst_state_list.try_borrow_data()?;
    let lst_state_list = try_lst_state_list(&lst_state_list_bytes)?;
    let dst_lst_state = lst_state_list[dst_lst_index]; // dst_lst_index checked above
    verify_lst_input_not_disabled(&dst_lst_state)?;

    let accounts_suffix_slice = accounts
        .get(START_REBALANCE_IX_ACCOUNTS_LEN..)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let src_dst_lst_indexes = SrcDstLstIndexes {
        src_lst_index,
        dst_lst_index,
    };

    let src_dst_lst_cpis = verify_src_dst_lst_sol_val_calc_cpis(
        VerifySrcDstLstSolValCalcCpiAccounts::from(actual),
        accounts_suffix_slice,
        *src_lst_calc_accs,
        src_dst_lst_indexes,
    )?;

    verify_has_succeeding_end_rebalance_ix(actual.instructions, *actual.dst_lst_mint.key)?;

    Ok((actual, src_dst_lst_cpis, src_dst_lst_indexes))
}

fn verify_has_succeeding_end_rebalance_ix(
    instructions_sysvar: &AccountInfo,
    dst_lst_mint: Pubkey,
) -> Result<(), ProgramError> {
    let mut next_ix_idx: usize = load_current_index_checked(instructions_sysvar)?.into();
    loop {
        next_ix_idx = next_ix_idx
            .checked_add(1)
            .ok_or(SControllerError::MathError)?;
        let next_ix = load_instruction_at_checked(next_ix_idx, instructions_sysvar)
            .map_err(|_| SControllerError::NoSucceedingEndRebalance)?;
        if is_end_rebalance_ix(&next_ix, dst_lst_mint) {
            break;
        }
    }
    Ok(())
}

const END_REBALANCE_IX_DST_LST_MINT_INDEX: usize = 4;

fn is_end_rebalance_ix(ix: &Instruction, dst_lst_mint: Pubkey) -> bool {
    let discm = match ix.data.first() {
        Some(d) => d,
        None => return false,
    };
    if *discm != END_REBALANCE_IX_DISCM {
        return false;
    }
    if ix.program_id != s_controller_lib::program::ID {
        return false;
    }
    let dst_lst_mint_account = match ix.accounts.get(END_REBALANCE_IX_DST_LST_MINT_INDEX) {
        Some(a) => a,
        None => return false,
    };
    dst_lst_mint_account.pubkey == dst_lst_mint
}
