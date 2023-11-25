use generic_pool_calculator_interface::{
    sol_to_lst_verify_account_keys, SolToLstAccounts, SolToLstIxArgs,
};
use generic_pool_calculator_lib::utils::{
    verify_no_stake_pool_prog_upgrade, VerifyNoStakePoolProgUpgradeArgs,
};
use sanctum_onchain_utils::utils::{load_accounts, log_and_return_wrong_acc_err};
use sol_value_calculator_onchain::process_sol_to_lst_unchecked;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use spl_calculator_interface::SplStakePool;
use spl_calculator_lib::{account_resolvers::SplLstSolCommonRootAccounts, SplStakePoolCalc};

pub fn process_sol_to_lst(
    accounts: &[AccountInfo],
    SolToLstIxArgs { amount }: SolToLstIxArgs,
) -> Result<(), ProgramError> {
    let stake_pool = verify(accounts)?;
    process_sol_to_lst_unchecked(&SplStakePoolCalc(stake_pool), amount)
}

fn verify(accounts: &[AccountInfo<'_>]) -> Result<SplStakePool, ProgramError> {
    let actual: SolToLstAccounts = load_accounts(accounts)?;

    let root_keys = SplLstSolCommonRootAccounts {
        spl_stake_pool: actual.pool_state,
        spl_stake_pool_prog: actual.pool_program,
    };
    let (intermediate, stake_pool) = root_keys.resolve()?;
    let expected = intermediate.resolve()?.into();

    sol_to_lst_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    // accounts should all be read-only, no need to verify_account_privileges

    verify_no_stake_pool_prog_upgrade(VerifyNoStakePoolProgUpgradeArgs {
        stake_pool_prog_data: actual.pool_program_data,
        calculator_state: actual.state,
    })?;

    Ok(stake_pool)
}
