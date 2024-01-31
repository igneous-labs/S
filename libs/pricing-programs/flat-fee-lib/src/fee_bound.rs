use flat_fee_interface::FlatFeeError;

const MAX_SIGNED_FEE_BPS: i16 = 10_000;
const MAX_UNSIGNED_FEE_BPS: u16 = 10_000;

pub fn verify_signed_fee_bps_bound(fee_bps_i16: i16) -> Result<(), FlatFeeError> {
    if !(-MAX_SIGNED_FEE_BPS..=MAX_SIGNED_FEE_BPS).contains(&fee_bps_i16) {
        return Err(FlatFeeError::SignedFeeOutOfBound);
    }
    Ok(())
}

pub fn verify_unsigned_fee_bps_bound(fee_bps_u16: u16) -> Result<(), FlatFeeError> {
    if !(0..=MAX_UNSIGNED_FEE_BPS).contains(&fee_bps_u16) {
        return Err(FlatFeeError::UnsignedFeeOutOfBound);
    }
    Ok(())
}
