use crate::{AmtsAfterFee, MathError, U64RatioFloor};

/// A fee ratio that should be <= 1.0.
/// amt_after_fees = floor(amt * (fee_denom - fee_num) / fee_denom),
/// effectively maximizing fees charged
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64FeeCeil<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub fee_num: N,
    pub fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeCeil<N, D> {
    /// Returns no fees charged if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if fee_num > fee_denom (fee > 100%)
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        let n: u128 = self.fee_num.into();
        let d: u128 = self.fee_denom.into();
        if n == 0 || d == 0 {
            return Ok(AmtsAfterFee {
                amt_after_fee: amt,
                fees_charged: 0,
            });
        }
        let num = d.checked_sub(n).ok_or(MathError)?;
        let amt_after_fee = U64RatioFloor {
            num,
            denom: self.fee_denom,
        }
        .apply(amt)?;
        let fees_charged = amt.checked_sub(amt_after_fee).ok_or(MathError)?;
        Ok(AmtsAfterFee {
            amt_after_fee,
            fees_charged,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.fee_num.into() <= self.fee_denom.into()
    }

    // TODO: pseudo_reverse()
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn u64_fee_lte_one()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeCeil<u64, u64> {
                U64FeeCeil { fee_num, fee_denom }
            }
    }

    proptest! {
        #[test]
        fn u64_fee_invariants(amt: u64, fee in u64_fee_lte_one()) {
            let AmtsAfterFee { amt_after_fee, fees_charged } = fee.apply(amt).unwrap();
            prop_assert!(amt_after_fee <= amt);
            prop_assert_eq!(amt, amt_after_fee + fees_charged);
        }
    }

    // TODO: round trip after pseudo_reverse()

    proptest! {
        #[test]
        fn u64_zero_denom(fee_num: u64, fee_denom in Just(0u64), amt: u64) {
            let fee = U64FeeCeil { fee_num, fee_denom };
            let amts_after_fee = fee.apply(amt).unwrap();

            prop_assert_eq!(amts_after_fee.amt_after_fee, amt);
            prop_assert_eq!(amts_after_fee.fees_charged, 0);
        }
    }

    proptest! {
        #[test]
        fn u64_zero_num(fee_num in Just(0u64), fee_denom: u64, amt: u64) {
            let fee = U64FeeCeil { fee_num, fee_denom };
            let amts_after_fee = fee.apply(amt).unwrap();

            prop_assert_eq!(amts_after_fee.amt_after_fee, amt);
            prop_assert_eq!(amts_after_fee.fees_charged, 0);
        }
    }

    prop_compose! {
        fn u64_smaller_larger()
            (boundary in any::<u64>())
            (smaller in 0..=boundary, larger in boundary..=u64::MAX) -> (u64, u64) {
                (smaller, larger)
            }
    }

    proptest! {
        #[test]
        fn valid_invalid((smaller, larger) in u64_smaller_larger()) {
            let valid = U64FeeCeil { fee_num: smaller, fee_denom: larger };
            prop_assert!(valid.is_valid());
            if smaller != larger {
                let invalid = U64FeeCeil { fee_num: larger, fee_denom: smaller };
                prop_assert!(!invalid.is_valid());
            }
        }
    }
}
