use generic_pool_calculator_interface::GenericPoolCalculatorError;
use generic_pool_calculator_lib::utils::checked_div_ceil;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{clock::Clock, program_error::ProgramError};
use spl_calculator_interface::{Fee, SplCalculatorError, SplStakePool};

#[derive(Debug, Clone)]
pub struct SplStakePoolCalc(pub SplStakePool);

pub fn verify_pool_updated_for_this_epoch(
    SplStakePool {
        last_update_epoch, ..
    }: &SplStakePool,
    clock: &Clock,
) -> Result<(), SplCalculatorError> {
    if *last_update_epoch == clock.epoch {
        Ok(())
    } else {
        Err(SplCalculatorError::PoolNotUpdated)
    }
}

/// Assumes:
/// - stake pool manager is always valid, so stake withdraw fee will always be charged
/// - stake pool always has active and transient stake, so withdraw_source != StakeWithdrawSource::ValidatorRemoval
/// - stake pool has been updated for this epoch
impl SolValueCalculator for SplStakePoolCalc {
    fn calc_lst_to_sol(&self, pool_tokens: u64) -> Result<u64, ProgramError> {
        let pool = &self.0;

        // Copied from:
        // https://github.com/solana-labs/solana-program-library/blob/52db94edb5571309dd7e5472c530ff56bcc30ae5/stake-pool/program/src/processor.rs#L3169-L3184
        let pool_tokens_fee = apply_fee(&pool.stake_withdrawal_fee, pool_tokens)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        let pool_tokens_burnt = pool_tokens
            .checked_sub(pool_tokens_fee)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        let withdraw_lamports = calc_lamports_withdraw_amount(pool, pool_tokens_burnt)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        Ok(withdraw_lamports)
    }

    /// TODO: make a prop-test for this: calc_sol_to_lst(calc_lst_to_sol(amt)) = amt
    fn calc_sol_to_lst(&self, withdraw_lamports: u64) -> Result<u64, ProgramError> {
        let pool = &self.0;

        let pool_tokens_burnt = calc_pool_tokens_burnt(pool, withdraw_lamports)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        let pool_tokens = reverse_fee(&pool.stake_withdrawal_fee, pool_tokens_burnt)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        Ok(pool_tokens)
    }
}

/// Copied from:
/// https://github.com/solana-labs/solana-program-library/blob/52db94edb5571309dd7e5472c530ff56bcc30ae5/stake-pool/program/src/state.rs#L941
/// Returns the amount to collect in fees (amt * numerator / denominator)
pub fn apply_fee(fee: &Fee, amt: u64) -> Option<u64> {
    if fee.denominator == 0 {
        return Some(0);
    }
    let res = (amt as u128)
        .checked_mul(fee.numerator as u128)?
        .checked_div(fee.denominator as u128)?;
    res.try_into().ok()
}

/// Inverse of [`apply_fee`], returns the amount before fees
pub fn reverse_fee(fee: &Fee, amt_after_deduct_fee: u64) -> Option<u64> {
    if fee.denominator == 0 || fee.numerator == 0 {
        return Some(amt_after_deduct_fee);
    }
    let res = (amt_after_deduct_fee as u128)
        .checked_mul(fee.denominator as u128)?
        .checked_div(fee.denominator.checked_sub(fee.numerator)? as u128)?;
    res.try_into().ok()
}

/// Copied from:
/// https://github.com/solana-labs/solana-program-library/blob/52db94edb5571309dd7e5472c530ff56bcc30ae5/stake-pool/program/src/state.rs#L178
pub fn calc_lamports_withdraw_amount(
    SplStakePool {
        total_lamports,
        pool_token_supply,
        ..
    }: &SplStakePool,
    pool_tokens: u64,
) -> Option<u64> {
    let numerator = (pool_tokens as u128).checked_mul(*total_lamports as u128)?;
    let denominator = *pool_token_supply as u128;
    if numerator < denominator || denominator == 0 {
        Some(0)
    } else {
        u64::try_from(numerator.checked_div(denominator)?).ok()
    }
}

/// Inverse of [`calc_lamports_withdraw_amount`]
pub fn calc_pool_tokens_burnt(
    SplStakePool {
        total_lamports,
        pool_token_supply,
        ..
    }: &SplStakePool,
    withdraw_lamports: u64,
) -> Option<u64> {
    if *total_lamports == 0 {
        return Some(0);
    }
    let pool_tokens_burnt = checked_div_ceil(
        (withdraw_lamports as u128).checked_mul(*pool_token_supply as u128)?,
        *total_lamports as u128,
    )?;
    pool_tokens_burnt.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use spl_calculator_interface::{AccountType, FutureEpochFee, Lockup};
    use test_utils::prop_assert_diff_at_most;

    prop_compose! {
        fn fee_rate_lte_one()
            (denominator in any::<u64>())
            (numerator in 0..=denominator, denominator in Just(denominator)) -> Fee {
                Fee { denominator, numerator }
            }
    }

    proptest! {
        #[test]
        fn fee_round_trip(amt: u64, fee in fee_rate_lte_one()) {
            let after = reverse_fee(&fee, amt - apply_fee(&fee, amt).unwrap()).unwrap();
            // TODO: fix math and determine suitable threshold
            prop_assert_diff_at_most!(after, amt, 5000);
        }
    }

    prop_compose! {
        fn spl_stake_pool_calc()
            (stake_withdrawal_fee in fee_rate_lte_one(), total_lamports: u64, pool_token_supply: u64) -> SplStakePoolCalc {
                let zero_fee = Fee { numerator: 0, denominator: 0 };
                SplStakePoolCalc(
                    SplStakePool {
                        total_lamports,
                        pool_token_supply,
                        stake_withdrawal_fee,
                        account_type: AccountType::StakePool,
                        manager: Default::default(),
                        staker: Default::default(),
                        stake_deposit_authority: Default::default(),
                        stake_withdraw_bump_seed: Default::default(),
                        validator_list: Default::default(),
                        reserve_stake: Default::default(),
                        pool_mint: Default::default(),
                        manager_fee_account: Default::default(),
                        token_program_id: Default::default(),
                        last_update_epoch: Default::default(),
                        lockup: Lockup {
                            unix_timestamp: Default::default(),
                            epoch: Default::default(),
                            custodian: Default::default(),
                        },
                        epoch_fee: zero_fee.clone(),
                        next_epoch_fee: FutureEpochFee::None,
                        preferred_deposit_validator_vote_address: Default::default(),
                        preferred_withdraw_validator_vote_address: Default::default(),
                        stake_deposit_fee: zero_fee.clone(),
                        next_stake_withdrawal_fee: FutureEpochFee::None,
                        stake_referral_fee: Default::default(),
                        sol_deposit_authority: Default::default(),
                        sol_deposit_fee: zero_fee.clone(),
                        sol_referral_fee: Default::default(),
                        sol_withdraw_authority: Default::default(),
                        sol_withdrawal_fee: zero_fee,
                        next_sol_withdrawal_fee: FutureEpochFee::None,
                        last_epoch_pool_token_supply: Default::default(),
                        last_epoch_total_lamports: Default::default()
                    }
                )
            }
    }

    prop_compose! {
        fn spl_stake_pool_and_lst_amount()
            (calc in spl_stake_pool_calc())
            (pool_token in 0..=calc.0.pool_token_supply, calc in Just(calc)) -> (u64, SplStakePoolCalc) {
                (pool_token, calc)
            }
    }

    proptest! {
        #[test]
        fn lst_sol_round_trip((pool_tokens, calc) in spl_stake_pool_and_lst_amount()) {
            let after = calc.calc_sol_to_lst(calc.calc_lst_to_sol(pool_tokens).unwrap()).unwrap();
            // TODO: fix math and determine suitable threshold
            prop_assert_diff_at_most!(after, pool_tokens, 10_000);
        }
    }
}
