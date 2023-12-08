use s_controller_interface::{
    withdraw_protocol_fees_verify_account_keys, withdraw_protocol_fees_verify_account_privileges,
    SControllerError, WithdrawProtocolFeesAccounts, WithdrawProtocolFeesIxArgs,
};
use s_controller_lib::{
    program::{POOL_STATE_BUMP, POOL_STATE_SEED},
    try_pool_state, WithdrawProtocolFeesFreeArgs,
};
use sanctum_onchain_utils::{
    token_program::{transfer_tokens_signed, TransferTokensAccounts},
    utils::{load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err},
};
use sanctum_utils::token::token_account_balance_program_agnostic;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::verify::verify_not_rebalancing_and_not_disabled;

pub fn process_withdraw_protocol_fees(
    accounts: &[AccountInfo],
    args: WithdrawProtocolFeesIxArgs,
) -> ProgramResult {
    let accounts = verify_withdraw_protocol_fees(accounts)?;

    if args.amount > token_account_balance_program_agnostic(accounts.protocol_fee_accumulator)? {
        return Err(SControllerError::NotEnoughFees.into());
    }

    transfer_tokens_signed(
        TransferTokensAccounts {
            from: accounts.protocol_fee_accumulator,
            to: accounts.withdraw_to,
            token_program: accounts.token_program,
            authority: accounts.pool_state,
        },
        args.amount,
        &[&[POOL_STATE_SEED, &[POOL_STATE_BUMP]]],
    )?;

    Ok(())
}

fn verify_withdraw_protocol_fees<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
) -> Result<WithdrawProtocolFeesAccounts<'a, 'info>, ProgramError> {
    let actual: WithdrawProtocolFeesAccounts = load_accounts(accounts)?;

    let free_args = WithdrawProtocolFeesFreeArgs {
        pool_state: actual.pool_state,
        withdraw_to: actual.withdraw_to,
        token_program: actual.token_program,
    };
    let expected = free_args.resolve()?;

    withdraw_protocol_fees_verify_account_keys(&actual, &expected)
        .map_err(log_and_return_wrong_acc_err)?;
    withdraw_protocol_fees_verify_account_privileges(&actual)
        .map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;

    verify_not_rebalancing_and_not_disabled(pool_state)?;

    Ok(actual)
}
