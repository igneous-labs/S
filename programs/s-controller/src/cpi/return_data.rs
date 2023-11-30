use solana_program::program::get_return_data;

/// Tries to read a little-endian u64 from solana return data
///
/// Returns `None` if
/// - no return data
/// - return data is not 8 bytes long
pub fn get_le_u64_return_data() -> Option<u64> {
    let (_pk, data) = get_return_data()?;
    let bytes: [u8; 8] = data.try_into().ok()?;
    Some(u64::from_le_bytes(bytes))
}
