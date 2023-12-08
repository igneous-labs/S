use flat_fee_interface::{
    remove_lst_verify_account_keys, remove_lst_verify_account_privileges, RemoveLstAccounts,
    RemoveLstKeys,
};
use flat_fee_lib::account_resolvers::RemoveLstFreeArgs;
use sanctum_onchain_utils::{
    system_program::{close_account, CloseAccountAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_remove_lst(accounts: &[AccountInfo]) -> ProgramResult {
    let RemoveLstAccounts {
        manager: _,
        fee_acc,
        state: _,
        // just realized closing maybe doesn't need system_program since
        // we're decrementing lamports directly. Not sure about
        // assigning account owner back to system_program
        // Need to test this.
        system_program: _,
        refund_rent_to,
    } = verify_remove_lst(accounts)?;

    close_account(CloseAccountAccounts {
        refund_rent_to,
        close: fee_acc,
    })?;

    Ok(())
}

fn verify_remove_lst<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<RemoveLstAccounts<'me, 'info>, ProgramError> {
    let actual: RemoveLstAccounts = load_accounts(accounts)?;

    let free_args = RemoveLstFreeArgs {
        refund_rent_to: *actual.refund_rent_to.key,
        state: actual.state,
        fee_acc: *actual.fee_acc.key,
    };
    let expected: RemoveLstKeys = free_args.resolve()?;

    remove_lst_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    remove_lst_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
