use flat_fee_interface::FlatFeeError;

const MAX_FEE_BPS: i16 = 10_000;

pub fn verify_signed_fee_bps_bound(fee_bps_i16: i16) -> Result<(), FlatFeeError> {
    if !(-MAX_FEE_BPS..=MAX_FEE_BPS).contains(&fee_bps_i16) {
        return Err(FlatFeeError::SignedFeeOutOfBound);
    }
    Ok(())
}
