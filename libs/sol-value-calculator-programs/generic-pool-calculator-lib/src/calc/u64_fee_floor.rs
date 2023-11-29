use generic_pool_calculator_interface::GenericPoolCalculatorError;

use crate::U64RatioFloor;

/// A fee ratio that should be <= 1.0
#[derive(Debug, Copy, Clone)]
pub struct U64FeeFloor<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub fee_num: N,
    pub fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeFloor<N, D> {
    /// Returns amt - (amt * fee_num // fee_denom)
    pub fn apply(&self, amt: u64) -> Result<u64, GenericPoolCalculatorError> {
        let deduct_amt = U64RatioFloor {
            num: self.fee_num,
            denom: self.fee_denom,
        }
        .apply(amt)?;
        amt.checked_sub(deduct_amt)
            .ok_or(GenericPoolCalculatorError::MathError)
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns `amt_after_apply` if fee_num == 0 || fee_denom == 0
    ///
    /// Returns 0 if fee_num >= fee_denom
    pub fn reverse(&self, amt_after_apply: u64) -> Result<u64, GenericPoolCalculatorError> {
        let n = self.fee_num.into();
        let d = self.fee_denom.into();
        if n == 0 || d == 0 {
            return Ok(amt_after_apply);
        }
        if n >= d {
            return Ok(0);
        }
        let y: u128 = amt_after_apply.into();
        let dy = y
            .checked_mul(d)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        let d_minus_n = d - n;
        let q_floor = dy / d_minus_n;

        q_floor
            .try_into()
            .map_err(|_e| GenericPoolCalculatorError::MathError)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn u64_fee_lte_one()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    proptest! {
        #[test]
        fn u64_fee_round_trip(amt: u64, fee in u64_fee_lte_one()) {
            let applied = fee.apply(amt).unwrap();

            let reversed = fee.reverse(applied).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            prop_assert_eq!(applied, apply_on_reversed);
        }
    }
}
