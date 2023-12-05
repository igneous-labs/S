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
    let checked = verify_remove_lst(accounts)?;
    process_remove_lst_unchecked(checked)
}

fn process_remove_lst_unchecked(
    RemoveLstAccounts {
        manager: _,
        fee_acc,
        state: _,
        system_program: _,
        refund_rent_to,
    }: RemoveLstAccounts,
) -> ProgramResult {
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
