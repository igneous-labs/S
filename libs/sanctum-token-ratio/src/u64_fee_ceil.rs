use crate::{AmtsAfterFees, MathError, U64RatioFloor};

/// A fee ratio that should be <= 1.0.
/// amt_after_fees = floor(amt * (fee_denom - fee_num) / fee_denom),
/// effectively maximizing fees charged
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64FeeCeil<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub fee_num: N,
    pub fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeCeil<N, D> {
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFees, MathError> {
        let num: u128 = self
            .fee_denom
            .into()
            .checked_sub(self.fee_num.into())
            .ok_or(MathError)?;
        let amt_after_fee = U64RatioFloor {
            num,
            denom: self.fee_denom,
        }
        .apply(amt)?;
        let fees_charged = amt.checked_sub(amt_after_fee).ok_or(MathError)?;
        Ok(AmtsAfterFees {
            amt_after_fee,
            fees_charged,
        })
    }
}
