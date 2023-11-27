use borsh::BorshDeserialize;
use generic_pool_calculator_interface::{lst_to_sol_verify_account_keys, LstToSolAccounts};
use generic_pool_calculator_lib::utils::{
    verify_no_stake_pool_prog_upgrade, VerifyNoStakePoolProgUpgradeArgs,
};
use marinade_calculator_interface::MarinadeState;
use marinade_calculator_lib::{MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS};
use sanctum_onchain_utils::utils::{load_accounts, log_and_return_wrong_acc_err};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

/// Assumes:
/// - LstToSolAccounts/Keys and SolToLstAccounts/Keys are identical
pub fn verify_lst_sol_common(accounts: &[AccountInfo<'_>]) -> Result<MarinadeState, ProgramError> {
    let actual: LstToSolAccounts = load_accounts(accounts)?;

    let expected = MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
        .resolve::<MarinadeSolValCalc>()
        .into();

    lst_to_sol_verify_account_keys(&actual, &expected).map_err(log_and_return_wrong_acc_err)?;
    // accounts should all be read-only, no need to verify_account_privileges

    verify_no_stake_pool_prog_upgrade(VerifyNoStakePoolProgUpgradeArgs {
        stake_pool_prog_data: actual.pool_program_data,
        calculator_state: actual.state,
    })?;

    Ok(MarinadeState::deserialize(
        &mut actual.pool_state.try_borrow_data()?.as_ref(),
    )?)
}
