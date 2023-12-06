/// amt_after_fees + fees_charged = amt_before_fees
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct AmtsAfterFee {
    pub amt_after_fee: u64,
    pub fees_charged: u64,
}
