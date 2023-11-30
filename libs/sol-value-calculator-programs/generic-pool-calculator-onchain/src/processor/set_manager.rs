use generic_pool_calculator_interface::{
    set_manager_verify_account_keys, set_manager_verify_account_privileges, SetManagerAccounts,
    SetManagerKeys,
};
use generic_pool_calculator_lib::{
    account_resolvers::SetManagerFreeArgs, utils::try_calculator_state_mut, GenericPoolSolValCalc,
};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

/// Call on resolved and checked SetManagerAccounts
pub fn process_set_manager_unchecked(
    SetManagerAccounts {
        manager: _,
        new_manager,
        state,
    }: SetManagerAccounts,
) -> Result<(), ProgramError> {
    let mut bytes = state.try_borrow_mut_data()?;
    let calc_state = try_calculator_state_mut(&mut bytes)?;
    calc_state.manager = *new_manager.key;
    Ok(())
}

pub fn verify_set_manager<'me, 'info, P: GenericPoolSolValCalc>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<SetManagerAccounts<'me, 'info>, ProgramError> {
    let actual: SetManagerAccounts = load_accounts(accounts)?;

    let root_keys = SetManagerFreeArgs {
        new_manager: *actual.new_manager.key,
        state: actual.state,
    };
    let expected: SetManagerKeys = root_keys.resolve::<P>()?;

    set_manager_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    set_manager_verify_account_privileges(&actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
