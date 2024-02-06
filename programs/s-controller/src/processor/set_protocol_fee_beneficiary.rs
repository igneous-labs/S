use s_controller_interface::{
    set_protocol_fee_beneficiary_verify_account_keys,
    set_protocol_fee_beneficiary_verify_account_privileges, SetProtocolFeeBeneficiaryAccounts,
};
use s_controller_lib::{try_pool_state, try_pool_state_mut, SetProtocolFeeBeneficiaryFreeArgs};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::verify::verify_not_rebalancing_and_not_disabled;

pub fn process_set_protocol_fee_beneficiary(accounts: &[AccountInfo]) -> ProgramResult {
    let checked = verify_set_protocol_fee_beneficiary(accounts)?;

    let mut pool_state_data = checked.pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_data)?;
    pool_state.protocol_fee_beneficiary = *checked.new_beneficiary.key;

    Ok(())
}

fn verify_set_protocol_fee_beneficiary<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
) -> Result<SetProtocolFeeBeneficiaryAccounts<'a, 'info>, ProgramError> {
    let actual: SetProtocolFeeBeneficiaryAccounts = load_accounts(accounts)?;

    let expected = SetProtocolFeeBeneficiaryFreeArgs {
        new_beneficiary: *actual.new_beneficiary.key,
        pool_state_acc: actual.pool_state,
    }
    .resolve()?;

    set_protocol_fee_beneficiary_verify_account_keys(actual, expected)
        .map_err(log_and_return_wrong_acc_err)?;
    set_protocol_fee_beneficiary_verify_account_privileges(actual)
        .map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    Ok(actual)
}
