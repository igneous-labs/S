/// Extremely fucked up TransactionReturnData truncates all rightmost zero bytes:
/// https://solana.stackexchange.com/questions/7141/program-return-data-to-client-error
pub fn zero_padded_return_data<const N: usize>(return_data: &[u8]) -> [u8; N] {
    let mut res = [0u8; N];
    let subslice = res.get_mut(..return_data.len()).unwrap();
    subslice.copy_from_slice(return_data);
    res
}
