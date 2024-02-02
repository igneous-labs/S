use borsh::BorshDeserialize;
use generic_pool_calculator_interface::{lst_to_sol_verify_account_keys, LstToSolAccounts};
use generic_pool_calculator_lib::utils::{
    verify_no_stake_pool_prog_upgrade, VerifyNoStakePoolProgUpgradeArgs,
};
use marinade_calculator_interface::MarinadeState;
use marinade_calculator_lib::{
    MarinadeSolValCalc, MarinadeStateCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use sanctum_misc_utils::{load_accounts, log_and_return_wrong_acc_err};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

const MARINADE_STATE_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];

/// Assumes:
/// - LstToSolAccounts/Keys and SolToLstAccounts/Keys are identical
pub fn verify_lst_sol_common(
    accounts: &[AccountInfo<'_>],
) -> Result<MarinadeStateCalc, ProgramError> {
    let actual: LstToSolAccounts = load_accounts(accounts)?;

    let expected = MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
        .resolve::<MarinadeSolValCalc>()
        .into();

    lst_to_sol_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    // accounts should all be read-only, no need to verify_account_privileges

    verify_no_stake_pool_prog_upgrade(VerifyNoStakePoolProgUpgradeArgs {
        stake_pool_prog_data: actual.pool_program_data,
        calculator_state: actual.state,
    })?;

    let state = MarinadeState::deserialize(&mut actual.pool_state.try_borrow_data()?.as_ref())?;
    if state.discriminant != MARINADE_STATE_DISCM {
        return Err(ProgramError::InvalidAccountData);
    }
    let calc: MarinadeStateCalc = state.into();

    calc.verify_can_withdraw_stake()?;

    Ok(calc)
}
