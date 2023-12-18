use lido_calculator_interface::{ExchangeRate, Lido, LidoCalculatorError};
use sanctum_token_ratio::{U64RatioFloor, U64ValueRange};
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{clock::Clock, program_error::ProgramError};

/// Parameters from Lido required to calculate SOL value.
/// Basically `ExchangeRate` but redeclared to derive Copy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LidoCalc {
    pub computed_in_epoch: u64,
    pub st_sol_supply: u64,
    pub sol_balance: u64,
}

impl From<&ExchangeRate> for LidoCalc {
    fn from(
        ExchangeRate {
            computed_in_epoch,
            st_sol_supply,
            sol_balance,
        }: &ExchangeRate,
    ) -> Self {
        Self {
            computed_in_epoch: *computed_in_epoch,
            st_sol_supply: *st_sol_supply,
            sol_balance: *sol_balance,
        }
    }
}

impl From<ExchangeRate> for LidoCalc {
    fn from(value: ExchangeRate) -> Self {
        (&value).into()
    }
}

impl From<&Lido> for LidoCalc {
    fn from(value: &Lido) -> Self {
        (&value.exchange_rate).into()
    }
}

impl From<Lido> for LidoCalc {
    fn from(value: Lido) -> Self {
        (&value).into()
    }
}

/// Reference
/// https://github.com/lidofinance/solido/blob/4e071bed845fca1e30215ec4e0be6b63e659bc18/program/src/processor.rs#L1034
impl LidoCalc {
    pub const fn verify_pool_updated_for_this_epoch(
        &self,
        clock: &Clock,
    ) -> Result<(), LidoCalculatorError> {
        if self.computed_in_epoch < clock.epoch {
            return Err(LidoCalculatorError::ExchangeRateNotUpdatedInThisEpoch);
        }
        Ok(())
    }

    pub const fn stlamports_to_lamports_ratio(&self) -> U64RatioFloor<u64, u64> {
        let Self {
            st_sol_supply,
            sol_balance,
            ..
        } = self;
        U64RatioFloor {
            num: *sol_balance,
            denom: *st_sol_supply,
        }
    }
}

/// Assumes:
/// - stake pool manager is always valid, so stake withdraw fee will always be charged
/// - stake pool always has active and transient stake, so withdraw_source != StakeWithdrawSource::ValidatorRemoval
/// - stake pool has been updated for this epoch
impl SolValueCalculator for LidoCalc {
    fn calc_lst_to_sol(&self, pool_tokens: u64) -> Result<U64ValueRange, ProgramError> {
        Ok(U64ValueRange::single(
            self.stlamports_to_lamports_ratio().apply(pool_tokens)?,
        ))
    }

    fn calc_sol_to_lst(&self, withdraw_lamports: u64) -> Result<U64ValueRange, ProgramError> {
        Ok(self
            .stlamports_to_lamports_ratio()
            .reverse(withdraw_lamports)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn lido_calc()
            (sol_balance: u64, st_sol_supply: u64) -> LidoCalc {
                LidoCalc {
                    st_sol_supply,
                    sol_balance,
                    computed_in_epoch: Default::default()
                }
            }
    }

    prop_compose! {
        fn lido_calc_and_stsol_amt()
            (calc in lido_calc())
            (stsol_amt in 0..=calc.st_sol_supply, calc in Just(calc)) -> (u64, LidoCalc) {
                (stsol_amt, calc)
            }
    }

    proptest! {
        #[test]
        fn lst_sol_round_trip((stsol_amt, calc) in lido_calc_and_stsol_amt()) {
            let U64ValueRange { min: sol_amt, max: max_sol_amt } = calc.calc_lst_to_sol(stsol_amt).unwrap();
            prop_assert_eq!(sol_amt, max_sol_amt);
            let U64ValueRange { min, max } = calc.calc_sol_to_lst(sol_amt).unwrap();
            prop_assert_eq!(calc.calc_lst_to_sol(min).unwrap().min, sol_amt);
            prop_assert_eq!(calc.calc_lst_to_sol(max).unwrap().min, sol_amt);
        }
    }
}
