use s_controller_interface::{
    remove_lst_verify_account_keys, remove_lst_verify_account_privileges, LstState,
    RemoveLstAccounts, RemoveLstIxArgs, SControllerError,
};
use s_controller_lib::{
    index_to_usize,
    program::{POOL_STATE_BUMP, POOL_STATE_SEED, PROTOCOL_FEE_BUMP, PROTOCOL_FEE_SEED},
    try_lst_state_list, try_pool_state, RemoveLstFreeArgs,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_token_lib::{
    close_token_account_invoke_signed, token_account_balance, CloseTokenAccountAccounts,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::{
    list_account::{remove_from_list_pda, RemoveFromListPdaAccounts},
    verify::verify_not_rebalancing_and_not_disabled,
};

pub fn process_remove_lst(accounts: &[AccountInfo], args: RemoveLstIxArgs) -> ProgramResult {
    let (accounts, lst_index) = verify_remove_lst(accounts, args)?;
    close_token_account_invoke_signed(
        CloseTokenAccountAccounts {
            account_to_close: accounts.protocol_fee_accumulator,
            authority: accounts.protocol_fee_accumulator_auth,
            token_program: accounts.lst_token_program,
            refund_rent_to: accounts.refund_rent_to,
        },
        &[&[PROTOCOL_FEE_SEED, &[PROTOCOL_FEE_BUMP]]],
    )?;

    close_token_account_invoke_signed(
        CloseTokenAccountAccounts {
            account_to_close: accounts.pool_reserves,
            authority: accounts.pool_state,
            token_program: accounts.lst_token_program,
            refund_rent_to: accounts.refund_rent_to,
        },
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;
    // Gotta put direct account lamport manipuation last after token program CPIs
    // because CPIs' lamport balance checks are broken:
    // https://github.com/solana-labs/solana/issues/9711
    remove_from_list_pda::<LstState>(
        RemoveFromListPdaAccounts {
            list_pda: accounts.lst_state_list,
            refund_rent_to: accounts.refund_rent_to,
        },
        lst_index,
    )
}

fn verify_remove_lst<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    RemoveLstIxArgs { lst_index }: RemoveLstIxArgs,
) -> Result<(RemoveLstAccounts<'a, 'info>, usize), ProgramError> {
    let lst_index = index_to_usize(lst_index)?;

    let actual: RemoveLstAccounts = load_accounts(accounts)?;

    let free_args = RemoveLstFreeArgs {
        lst_index,
        refund_rent_to: *actual.refund_rent_to.key,
        pool_state: actual.pool_state,
        lst_state_list: actual.lst_state_list,
        lst_mint: actual.lst_mint,
    };
    let expected = free_args.resolve()?;

    remove_lst_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    remove_lst_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    let lst_state_list_acc_data = actual.lst_state_list.try_borrow_data()?;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;
    let lst_state = lst_state_list[lst_index]; // index checked during accounts resolution

    if lst_state.sol_value != 0
        || token_account_balance(actual.pool_reserves)? != 0
        || token_account_balance(actual.protocol_fee_accumulator)? != 0
    {
        return Err(SControllerError::LstStillHasValue.into());
    }

    Ok((actual, lst_index))
}
