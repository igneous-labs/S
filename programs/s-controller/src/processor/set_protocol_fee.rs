use s_controller_interface::{
    set_protocol_fee_verify_account_keys, set_protocol_fee_verify_account_privileges,
    SControllerError, SetProtocolFeeAccounts, SetProtocolFeeIxArgs,
};
use s_controller_lib::{try_pool_state, try_pool_state_mut, SetProtocolFeeFreeArgs};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_token_ratio::BPS_DENOMINATOR;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::verify::verify_not_rebalancing_and_not_disabled;

pub fn process_set_protocol_fee(
    accounts: &[AccountInfo],
    args: SetProtocolFeeIxArgs,
) -> ProgramResult {
    let (
        accounts,
        SetProtocolFeeIxArgs {
            new_trading_protocol_fee_bps,
            new_lp_protocol_fee_bps,
        },
    ) = verify_set_protocol_fee(accounts, args)?;

    let mut pool_state_bytes = accounts.pool_state.try_borrow_mut_data()?;
    let pool_state = try_pool_state_mut(&mut pool_state_bytes)?;

    if let Some(new_trading_protocol_fee_bps) = new_trading_protocol_fee_bps {
        pool_state.trading_protocol_fee_bps = new_trading_protocol_fee_bps;
    }
    if let Some(new_lp_protocol_fee_bps) = new_lp_protocol_fee_bps {
        pool_state.lp_protocol_fee_bps = new_lp_protocol_fee_bps;
    }

    Ok(())
}

fn verify_set_protocol_fee<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    SetProtocolFeeIxArgs {
        new_trading_protocol_fee_bps,
        new_lp_protocol_fee_bps,
    }: SetProtocolFeeIxArgs,
) -> Result<(SetProtocolFeeAccounts<'a, 'info>, SetProtocolFeeIxArgs), ProgramError> {
    let actual: SetProtocolFeeAccounts = load_accounts(accounts)?;

    let free_args = SetProtocolFeeFreeArgs {
        pool_state_acc: actual.pool_state,
    };
    let expected = free_args.resolve()?;

    set_protocol_fee_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    set_protocol_fee_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    for fee_bps in [new_lp_protocol_fee_bps, new_trading_protocol_fee_bps]
        .into_iter()
        .flatten()
    {
        if fee_bps > BPS_DENOMINATOR {
            return Err(SControllerError::FeeTooHigh.into());
        }
    }

    Ok((
        actual,
        SetProtocolFeeIxArgs {
            new_trading_protocol_fee_bps,
            new_lp_protocol_fee_bps,
        },
    ))
}
