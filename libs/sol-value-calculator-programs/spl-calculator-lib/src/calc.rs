use sanctum_token_ratio::{
    CeilDiv, FloorDiv, MathError, ReversibleFee, ReversibleRatio, U64FeeRatio, U64Ratio,
    U64ValueRange,
};
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::program_error::ProgramError;
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
        this_epoch: u64,
    ) -> Result<(), SplCalculatorError> {
        if self.last_update_epoch == this_epoch {
            Ok(())
        } else {
            Err(SplCalculatorError::PoolNotUpdated)
        }
    }

    pub const fn lst_to_lamports_ratio(&self) -> FloorDiv<U64Ratio<u64, u64>> {
        let Self {
            total_lamports,
            pool_token_supply,
            ..
        } = self;
        FloorDiv(U64Ratio {
            num: *total_lamports,
            denom: *pool_token_supply,
        })
    }

    pub fn stake_withdrawal_fee(&self) -> Result<CeilDiv<U64FeeRatio<u64, u64>>, MathError> {
        U64FeeRatio::try_from_fee_num_and_denom(
            self.stake_withdrawal_fee_numerator,
            self.stake_withdrawal_fee_denominator,
        )
        .map(CeilDiv)
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
        let aaf = self.stake_withdrawal_fee()?.apply(pool_tokens)?;
        let pool_tokens_burnt = aaf.amt_after_fee();
        let withdraw_lamports = self.lst_to_lamports_ratio().apply(pool_tokens_burnt)?;
        Ok(U64ValueRange::single(withdraw_lamports))
    }

    fn calc_sol_to_lst(&self, withdraw_lamports: u64) -> Result<U64ValueRange, ProgramError> {
        let r = self.lst_to_lamports_ratio().reverse(withdraw_lamports)?;
        let fee = self.stake_withdrawal_fee()?;
        let min = fee.reverse_from_amt_after_fee(r.get_min())?.get_min();
        let max = fee.reverse_from_amt_after_fee(r.get_max())?.get_max();
        Ok(U64ValueRange::try_from_min_max(min, max)?)
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
            let r = calc.calc_lst_to_sol(pool_tokens).unwrap();
            let sol_amt = r.get_min();
            let max_sol_amt = r.get_max();
            prop_assert_eq!(sol_amt, max_sol_amt);
            let r = calc.calc_sol_to_lst(sol_amt).unwrap();
            let min = r.get_min();
            let max = r.get_max();

            // round trip from min should not exceed original
            let min_round_trip = calc.calc_lst_to_sol(min).unwrap();
            prop_assert!(sol_amt >= min_round_trip.get_min(), "{sol_amt} {}", min_round_trip.get_min());
            prop_assert!(sol_amt >= min_round_trip.get_max(), "{sol_amt} {}", min_round_trip.get_max());

            // round trip from max should not be smaller than original
            let max_round_trip = calc.calc_lst_to_sol(max).unwrap();
            prop_assert!(sol_amt <= max_round_trip.get_min(), "{sol_amt} {}", max_round_trip.get_min());
            prop_assert!(sol_amt <= max_round_trip.get_max(), "{sol_amt} {}", max_round_trip.get_max());
        }
    }
}
