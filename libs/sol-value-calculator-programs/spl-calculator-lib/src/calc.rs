use sanctum_token_ratio::{AmtsAfterFee, MathError, U64RatioFloor, U64ValueRange};
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

    // TODO: add this math to sanctum-token-ratio. Idk what to call it tho, since
    // `U64FeeCeil` is already taken. `U64FeeFloor::apply_ceil()`?? totally not confusing lol
    //
    // Math from latest changes due to SPL Halborn audit:
    // https://github.com/solana-labs/solana-program-library/pull/6153

    pub fn apply_withdrawal_fee(&self, pool_tokens: u64) -> Result<AmtsAfterFee, MathError> {
        let n: u128 = self.stake_withdrawal_fee_numerator.into();
        let d: u128 = self.stake_withdrawal_fee_denominator.into();
        let x: u128 = pool_tokens.into();
        if d == 0 {
            return Ok(AmtsAfterFee {
                amt_after_fee: pool_tokens,
                fee_charged: 0,
            });
        }
        let num = x
            .checked_mul(n)
            .and_then(|xn| xn.checked_add(d))
            .and_then(|xn_plus_d| xn_plus_d.checked_sub(1))
            .ok_or(MathError)?;
        let fee_charged = (num / d).try_into().map_err(|_e| MathError)?;
        let amt_after_fee = pool_tokens.checked_sub(fee_charged).ok_or(MathError)?;
        Ok(AmtsAfterFee {
            amt_after_fee,
            fee_charged,
        })
    }

    // Reversing from amt after fee:
    // let y = amt after fee, x = input we're trying to find, n = fee numerator, d = fee denominator
    //
    // y = x - floor[(xn + d - 1) / d]
    // x - y = floor[(xn + d - 1) / d]
    // x - y <= (xn + d - 1) / d < x - y + 1
    //
    // LHS (max):
    // dx - dy <= xn + d - 1
    // dx - xn <= dy + d - 1
    // x(d - n) <= dy + d - 1
    // x <= (dy - 1 + d) / (d - n)
    //
    // RHS (min):
    // xn + d - 1 < dx - dy + d
    // dy - 1 < dx - xn
    // dy - 1 < x(d - n)
    // (dy - 1) / (d - n) < x
    //
    // Returns:
    // - U64ValueRange::single(amt_after_fee) if n == 0 || d == 0 (no fees)
    // - MathError if n >= d (>=100% fees)
    // - max range exclusive if (dy - 1 + d) is not divisible by (d - n)
    pub fn reverse_withdrawal_fee_from_amt_after_fee(
        &self,
        amt_after_fee: u64,
    ) -> Result<U64ValueRange, MathError> {
        let n: u128 = self.stake_withdrawal_fee_numerator.into();
        let d: u128 = self.stake_withdrawal_fee_denominator.into();
        let y: u128 = amt_after_fee.into();

        if n == 0 || d == 0 {
            return Ok(U64ValueRange::single(amt_after_fee));
        }
        if n >= d {
            return Err(MathError);
        }

        // unchecked-sub safety: n < d
        let d_minus_n = d - n;

        let dy_minus_1 = d
            .checked_mul(y)
            .and_then(|dy| dy.checked_sub(1))
            .ok_or(MathError)?;

        // unchecked-div safety: d_minus_n > 0 since n < d
        let min = dy_minus_1 / d_minus_n;

        let dy_minus_1_plus_d = dy_minus_1.checked_add(d).ok_or(MathError)?;
        // dy_minus_1_plus_d.ceil_div(d_minus_n)
        let num = dy_minus_1_plus_d
            .checked_add(d_minus_n)
            .and_then(|a| a.checked_sub(1))
            .ok_or(MathError)?;
        let max = num / d_minus_n;

        if min > max {
            return Err(MathError);
        }
        Ok(U64ValueRange {
            min: min.try_into().map_err(|_e| MathError)?,
            max: max.try_into().map_err(|_e| MathError)?,
        })
    }
}

/// Assumes:
/// - stake pool manager is always valid, so stake withdraw fee will always be charged
/// - stake pool always has active and transient stake, so withdraw_source != StakeWithdrawSource::ValidatorRemoval
/// - stake pool has been updated for this epoch
impl SolValueCalculator for SplStakePoolCalc {
    // Reference:
    // https://github.com/solana-labs/solana-program-library/blob/c225e8025f7dbf3134683ec387671b9251a4606c/stake-pool/program/src/processor.rs#L3169
    // applies fees on pool_tokens first and then converts amt_after_fee to lamports equivalent
    fn calc_lst_to_sol(&self, pool_tokens: u64) -> Result<U64ValueRange, ProgramError> {
        let AmtsAfterFee {
            amt_after_fee: pool_tokens_burnt,
            ..
        } = self.apply_withdrawal_fee(pool_tokens)?;
        let withdraw_lamports = self.lst_to_lamports_ratio().apply(pool_tokens_burnt)?;
        Ok(U64ValueRange::single(withdraw_lamports))
    }

    fn calc_sol_to_lst(&self, withdraw_lamports: u64) -> Result<U64ValueRange, ProgramError> {
        let U64ValueRange { min, max } = self.lst_to_lamports_ratio().reverse(withdraw_lamports)?;
        let U64ValueRange { min, .. } = self.reverse_withdrawal_fee_from_amt_after_fee(min)?;
        let U64ValueRange { max, .. } = self.reverse_withdrawal_fee_from_amt_after_fee(max)?;
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
