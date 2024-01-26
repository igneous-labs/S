use sanctum_token_ratio::{AmtsAfterFee, MathError, U64FeeFloor, U64RatioFloor, U64ValueRange};
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{clock::Clock, program_error::ProgramError};
use spl_calculator_interface::{Fee, SplCalculatorError, SplStakePool};

/// Parameters from SplStakePool required to calculate SOL value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SplStakePoolCalc {
    pub last_update_epoch: u64,
    pub total_lamports: u64,
    pub pool_token_supply: u64,
    pub stake_withdrawal_fee_numerator: u64,
    pub stake_withdrawal_fee_denominator: u64,
}

impl From<&SplStakePool> for SplStakePoolCalc {
    fn from(
        SplStakePool {
            total_lamports,
            pool_token_supply,
            last_update_epoch,
            stake_withdrawal_fee:
                Fee {
                    denominator,
                    numerator,
                },
            ..
        }: &SplStakePool,
    ) -> Self {
        Self {
            last_update_epoch: *last_update_epoch,
            total_lamports: *total_lamports,
            pool_token_supply: *pool_token_supply,
            stake_withdrawal_fee_numerator: *numerator,
            stake_withdrawal_fee_denominator: *denominator,
        }
    }
}

impl From<SplStakePool> for SplStakePoolCalc {
    fn from(value: SplStakePool) -> Self {
        (&value).into()
    }
}

impl SplStakePoolCalc {
    pub const fn verify_pool_updated_for_this_epoch(
        &self,
        clock: &Clock,
    ) -> Result<(), SplCalculatorError> {
        if self.last_update_epoch == clock.epoch {
            Ok(())
        } else {
            Err(SplCalculatorError::PoolNotUpdated)
        }
    }

    pub const fn lst_to_lamports_ratio(&self) -> U64RatioFloor<u64, u64> {
        let Self {
            total_lamports,
            pool_token_supply,
            ..
        } = self;
        U64RatioFloor {
            num: *total_lamports,
            denom: *pool_token_supply,
        }
    }

    pub const fn stake_withdrawal_fee(&self) -> U64FeeFloor<u64, u64> {
        let Self {
            stake_withdrawal_fee_numerator,
            stake_withdrawal_fee_denominator,
            ..
        } = self;
        U64FeeFloor {
            fee_num: *stake_withdrawal_fee_numerator,
            fee_denom: *stake_withdrawal_fee_denominator,
        }
    }
}

/// Assumes:
/// - stake pool manager is always valid, so stake withdraw fee will always be charged
/// - stake pool always has active and transient stake, so withdraw_source != StakeWithdrawSource::ValidatorRemoval
/// - stake pool has been updated for this epoch
impl SolValueCalculator for SplStakePoolCalc {
    fn calc_lst_to_sol(&self, pool_tokens: u64) -> Result<U64ValueRange, ProgramError> {
        let AmtsAfterFee {
            amt_after_fee: pool_tokens_burnt,
            ..
        } = self.stake_withdrawal_fee().apply(pool_tokens)?;
        let withdraw_lamports = self.lst_to_lamports_ratio().apply(pool_tokens_burnt)?;
        Ok(U64ValueRange::single(withdraw_lamports))
    }

    fn calc_sol_to_lst(&self, withdraw_lamports: u64) -> Result<U64ValueRange, ProgramError> {
        let U64ValueRange { min, max } = self.lst_to_lamports_ratio().reverse(withdraw_lamports)?;
        let fee = self.stake_withdrawal_fee();
        let U64ValueRange { min, .. } = fee.reverse_from_amt_after_fee(min)?;
        let U64ValueRange { max, .. } = fee.reverse_from_amt_after_fee(max)?;
        if min > max {
            return Err(MathError.into());
        }
        Ok(U64ValueRange { min, max })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn fee_rate_lte_one()
            (denominator in any::<u64>())
            (numerator in 0..=denominator, denominator in Just(denominator)) -> Fee {
                Fee { denominator, numerator }
            }
    }

    prop_compose! {
        fn spl_stake_pool_calc()
            (Fee { denominator, numerator } in fee_rate_lte_one(), total_lamports: u64, pool_token_supply: u64) -> SplStakePoolCalc {
                SplStakePoolCalc {
                    last_update_epoch: 0,
                    total_lamports,
                    pool_token_supply,
                    stake_withdrawal_fee_numerator: numerator,
                    stake_withdrawal_fee_denominator: denominator,
                }
            }
    }

    prop_compose! {
        fn spl_stake_pool_and_lst_amount()
            (calc in spl_stake_pool_calc())
            (pool_token in 0..=calc.pool_token_supply, calc in Just(calc)) -> (u64, SplStakePoolCalc) {
                (pool_token, calc)
            }
    }

    proptest! {
        #[test]
        fn lst_sol_round_trip((pool_tokens, calc) in spl_stake_pool_and_lst_amount()) {
            let U64ValueRange { min: sol_amt, max: max_sol_amt } = calc.calc_lst_to_sol(pool_tokens).unwrap();
            prop_assert_eq!(sol_amt, max_sol_amt);
            let U64ValueRange { min, max } = calc.calc_sol_to_lst(sol_amt).unwrap();

            // round trip from min should not exceed original
            let min_round_trip = calc.calc_lst_to_sol(min).unwrap();
            prop_assert!(sol_amt >= min_round_trip.min, "{sol_amt} {}", min_round_trip.min);
            prop_assert!(sol_amt >= min_round_trip.max, "{sol_amt} {}", min_round_trip.max);

            // round trip from max should not be smaller than original
            let max_round_trip = calc.calc_lst_to_sol(max).unwrap();
            prop_assert!(sol_amt <= max_round_trip.min, "{sol_amt} {}", max_round_trip.min);
            prop_assert!(sol_amt <= max_round_trip.max, "{sol_amt} {}", max_round_trip.max);
        }
    }
}
