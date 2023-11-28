use generic_pool_calculator_interface::GenericPoolCalculatorError;

/// A ratio that is applied to a u64 token amount
#[derive(Debug, Copy, Clone)]
pub struct U64RatioFloor<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub num: N,
    pub denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64RatioFloor<N, D> {
    /// Returns amt * num // denom
    /// Returns 0 if denominator == 0
    pub fn apply(&self, amt: u64) -> Result<u64, GenericPoolCalculatorError> {
        let d = self.denom.into();
        if d == 0 {
            return Ok(0);
        }
        let n = self.num.into();
        let x: u128 = amt.into();
        x.checked_mul(n)
            .map(|nx| nx / d) // d != 0
            .and_then(|res| res.try_into().ok())
            .ok_or(GenericPoolCalculatorError::MathError)
    }

    /// Returns 0 if denominator == 0 || numerator == 0
    pub fn reverse(&self, amt_after_apply: u64) -> Result<u64, GenericPoolCalculatorError> {
        let d = self.denom.into();
        let n = self.num.into();
        if d == 0 || n == 0 {
            return Ok(0);
        }
        let y: u128 = amt_after_apply.into();
        let dy = y
            .checked_mul(d)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        // n != 0, d != 0
        let q_floor: u64 = (dy / n)
            .try_into()
            .map_err(|_e| GenericPoolCalculatorError::MathError)?;
        let r = dy % n;

        if r == 0 {
            return Ok(q_floor);
        }

        let d_plus_r = d
            .checked_add(r)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        let q_ceil = q_floor
            .checked_add(1)
            .ok_or(GenericPoolCalculatorError::MathError)?;

        if d_plus_r >= n || d >= n {
            return Ok(q_ceil);
        }

        let r_plus_r = r
            .checked_add(r)
            .ok_or(GenericPoolCalculatorError::MathError)?;

        // d < n
        let res = if r_plus_r <= (n - d) { q_floor } else { q_ceil };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn u64_ratio_gte_one()
            (denom in any::<u64>())
            (num in denom..=u64::MAX, denom in Just(denom)) -> U64RatioFloor<u64, u64> {
                U64RatioFloor { num, denom }
            }
    }

    prop_compose! {
        fn u64_ratio_gte_one_and_overflow_max_limit()
            (u64ratio in u64_ratio_gte_one()) -> (u64, U64RatioFloor<u64, u64>) {
                if u64ratio.num == 0 {
                    return (u64::MAX, u64ratio);
                }
                let max_limit = u64ratio.denom as u128 * u64::MAX as u128 / u64ratio.num as u128;
                if max_limit >= u64::MAX as u128 {
                    return (u64::MAX, u64ratio);
                }
                (max_limit.try_into().unwrap(), u64ratio)
            }
    }

    prop_compose! {
        fn u64_ratio_gte_one_amt_no_overflow()
            ((maxlimit, u64ratio) in u64_ratio_gte_one_and_overflow_max_limit())
            (amt in 0..=maxlimit, u64ratio in Just(u64ratio)) -> (u64, U64RatioFloor<u64, u64>) {
                (amt, u64ratio)
            }
    }

    proptest! {
        #[test]
        fn u64_ratio_round_trip((amt, ratio) in u64_ratio_gte_one_amt_no_overflow()) {
            let applied = ratio.apply(amt).unwrap();
            let reversed = ratio.reverse(applied).unwrap();

            prop_assert_eq!(reversed, amt);
        }
    }
}
