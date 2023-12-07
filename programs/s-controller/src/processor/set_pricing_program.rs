use s_controller_interface::{
    set_pricing_program_verify_account_keys, set_pricing_program_verify_account_privileges,
    SetPricingProgramAccounts,
};
use s_controller_lib::{try_pool_state_mut, SetPricingProgramFreeArgs};
use sanctum_onchain_utils::utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

pub fn process_set_pricing_program(accounts: &[AccountInfo]) -> ProgramResult {
    let SetPricingProgramAccounts {
        admin: _,
        new_pricing_program,
        pool_state,
    } = verify_set_pricing_program(accounts)?;

    let mut pool_state_bytes = pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_bytes)?;

    pool_state.pricing_program = *new_pricing_program.key;

    Ok(())
}

fn verify_set_pricing_program<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<SetPricingProgramAccounts<'me, 'info>, ProgramError> {
    let actual: SetPricingProgramAccounts = load_accounts(accounts)?;

    let free_args = SetPricingProgramFreeArgs {
        new_pricing_program: *actual.new_pricing_program.key,
        pool_state_acc: actual.pool_state,
    };
    let expected = free_args.resolve()?;

    set_pricing_program_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    set_pricing_program_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
