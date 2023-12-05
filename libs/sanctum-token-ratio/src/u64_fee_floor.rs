use crate::{AmtsAfterFee, MathError, U64RatioFloor};

/// A fee ratio that should be <= 1.0.
/// fee_amt = floor(amt * fee_num / fee_denom)
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64FeeFloor<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub fee_num: N,
    pub fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeFloor<N, D> {
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        let fees_charged = U64RatioFloor {
            num: self.fee_num,
            denom: self.fee_denom,
        }
        .apply(amt)?;
        let amt_after_fee = amt.checked_sub(fees_charged).ok_or(MathError)?;
        Ok(AmtsAfterFee {
            amt_after_fee,
            fees_charged,
        })
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns `amt_after_apply` if fee_num == 0 || fee_denom == 0
    ///
    /// Returns 0 if fee_num >= fee_denom
    pub fn pseudo_reverse(&self, amt_after_fee: u64) -> Result<u64, MathError> {
        let n = self.fee_num.into();
        let d = self.fee_denom.into();
        if n == 0 || d == 0 {
            return Ok(amt_after_fee);
        }
        if n >= d {
            return Ok(0);
        }
        let y: u128 = amt_after_fee.into();
        let dy = y.checked_mul(d).ok_or(MathError)?;
        let d_minus_n = d - n;
        let q_floor = dy / d_minus_n;

        q_floor.try_into().map_err(|_e| MathError)
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
            let AmtsAfterFee { amt_after_fee, .. } = fee.apply(amt).unwrap();

            let reversed = fee.pseudo_reverse(amt_after_fee).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            prop_assert_eq!(amt_after_fee, apply_on_reversed.amt_after_fee);
        }
    }
}
