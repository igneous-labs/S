use s_controller_interface::{
    remove_lst_verify_account_keys, remove_lst_verify_account_privileges, LstState,
    RemoveLstAccounts, RemoveLstIxArgs, SControllerError,
};
use s_controller_lib::{
    program::{POOL_STATE_BUMP, POOL_STATE_SEED, PROTOCOL_FEE_BUMP, PROTOCOL_FEE_SEED},
    try_lst_state_list, RemoveLstFreeArgs,
};
use sanctum_onchain_utils::{
    token_program::{close_token_account_signed, CloseTokenAccountAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use sanctum_utils::token::token_account_balance_program_agnostic;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::list_account::{remove_from_list_pda, RemoveFromListPdaAccounts};

pub fn process_remove_lst(accounts: &[AccountInfo], args: RemoveLstIxArgs) -> ProgramResult {
    let (accounts, lst_index) = verify_remove_lst(accounts, args)?;
    close_token_account_signed(
        CloseTokenAccountAccounts {
            account_to_close: accounts.protocol_fee_accumulator,
            authority: accounts.protocol_fee_accumulator_auth,
            token_program: accounts.lst_token_program,
            refund_rent_to: accounts.refund_rent_to,
        },
        &[&[PROTOCOL_FEE_SEED, &[PROTOCOL_FEE_BUMP]]],
    )?;

    close_token_account_signed(
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
    let actual: RemoveLstAccounts = load_accounts(accounts)?;

    let free_args = RemoveLstFreeArgs {
        lst_index,
        refund_rent_to: *actual.refund_rent_to.key,
        pool_state: actual.pool_state,
        lst_state_list: actual.lst_state_list,
        lst_mint: actual.lst_mint,
    };
    let expected = free_args.resolve()?;

    remove_lst_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    remove_lst_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    let lst_index: usize = lst_index
        .try_into()
        .map_err(|_e| SControllerError::MathError)?;

    let lst_state_list_acc_data = actual.lst_state_list.try_borrow_data()?;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;
    let lst_state = lst_state_list[lst_index]; // index checked during accounts resolution

    if lst_state.sol_value != 0
        || token_account_balance_program_agnostic(actual.pool_reserves)? != 0
        || token_account_balance_program_agnostic(actual.protocol_fee_accumulator)? != 0
    {
        return Err(SControllerError::LstStillHasValue.into());
    }

    Ok((actual, lst_index))
}
