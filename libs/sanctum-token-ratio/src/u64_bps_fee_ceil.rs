use crate::{AmtsAfterFees, MathError, U64FeeCeil, BPS_DENOMINATOR};

/// A bps fee to charge where value <= 10_000
/// amt_after_fees = floor(amt * (10_000 - fee_num) / 10_000),
/// effectively maximizing fees charged
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64BpsFeeCeil<N: Copy + Into<u128>>(pub N);

impl<N: Copy + Into<u128>> U64BpsFeeCeil<N> {
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFees, MathError> {
        U64FeeCeil {
            fee_num: self.0,
            fee_denom: BPS_DENOMINATOR,
        }
        .apply(amt)
    }
}
