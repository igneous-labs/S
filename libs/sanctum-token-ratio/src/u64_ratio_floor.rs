use crate::MathError;

/// A ratio that is applied to a u64 token amount
/// with floor division
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64RatioFloor<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub num: N,
    pub denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64RatioFloor<N, D> {
    /// Returns amt * num // denom
    /// Returns 0 if denominator == 0
    pub fn apply(&self, amt: u64) -> Result<u64, MathError> {
        let d = self.denom.into();
        if d == 0 {
            return Ok(0);
        }
        let n = self.num.into();
        let x: u128 = amt.into();
        x.checked_mul(n)
            .map(|nx| nx / d) // d != 0
            .and_then(|res| res.try_into().ok())
            .ok_or(MathError)
    }

    /// Returns 0 if denominator == 0 || numerator == 0
    pub fn pseudo_reverse(&self, amt_after_apply: u64) -> Result<u64, MathError> {
        let d = self.denom.into();
        let n = self.num.into();
        if d == 0 || n == 0 {
            return Ok(0);
        }
        let y: u128 = amt_after_apply.into();
        let dy = y.checked_mul(d).ok_or(MathError)?;
        // n != 0, d != 0
        let q_floor: u64 = (dy / n).try_into().map_err(|_e| MathError)?;
        let r = dy % n;

        if r == 0 {
            return Ok(q_floor);
        }

        let d_plus_r = d.checked_add(r).ok_or(MathError)?;
        let q_ceil = q_floor.checked_add(1).ok_or(MathError)?;

        if d_plus_r >= n || d >= n {
            return Ok(q_ceil);
        }

        let r_plus_r = r.checked_add(r).ok_or(MathError)?;

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
        fn u64_ratio_lte_one()
            (denom in any::<u64>())
            (num in 0..=denom, denom in Just(denom)) -> U64RatioFloor<u64, u64> {
                U64RatioFloor { num, denom }
            }
    }

    prop_compose! {
        /// max_limit is the max number that ratio can be applied to without overflowing u64
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
        fn u64_ratio_gte_one_round_trip((amt, ratio) in u64_ratio_gte_one_amt_no_overflow()) {
            let applied = ratio.apply(amt).unwrap();
            let reversed = ratio.pseudo_reverse(applied).unwrap();
            prop_assert_eq!(reversed, amt);
        }
    }

    proptest! {
        #[test]
        fn u64_ratio_lte_one_round_trip(amt: u64, ratio in u64_ratio_lte_one()) {
            let applied = ratio.apply(amt).unwrap();
            let reversed = ratio.pseudo_reverse(applied).unwrap();
            // will not always be eq due to floor
            prop_assert!(reversed <= amt);
            // but make sure they applying the ratio again yields the same result
            let apply_on_reversed = ratio.apply(reversed).unwrap();
            prop_assert_eq!(applied, apply_on_reversed);
        }
    }

    proptest! {
        #[test]
        fn u64_zero_denom(num: u64, denom in Just(0u64), amt: u64) {
            let ratio = U64RatioFloor { num, denom };
            prop_assert_eq!(ratio.apply(amt).unwrap(), 0);
            prop_assert_eq!(ratio.pseudo_reverse(amt).unwrap(), 0);
        }
    }

    proptest! {
        #[test]
        fn u64_zero_num(num in Just(0u64), denom: u64, amt: u64) {
            let ratio = U64RatioFloor { num, denom };
            prop_assert_eq!(ratio.apply(amt).unwrap(), 0);
            prop_assert_eq!(ratio.pseudo_reverse(amt).unwrap(), 0);
        }
    }
}
