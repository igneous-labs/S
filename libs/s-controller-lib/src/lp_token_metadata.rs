use solana_program::program_error::ProgramError;
use spl_token_metadata_interface::state::TokenMetadata;

use crate::{
    DEFAULT_LP_TOKEN_METADATA_NAME, DEFAULT_LP_TOKEN_METADATA_SYMBOL, DEFAULT_LP_TOKEN_METADATA_URI,
};

/// Returns the number of bytes of the TLV entry of the initial token metadata
/// to write into the mint account
pub fn initial_token_metadata_size() -> Result<usize, ProgramError> {
    TokenMetadata {
        name: DEFAULT_LP_TOKEN_METADATA_NAME.into(),
        uri: DEFAULT_LP_TOKEN_METADATA_URI.into(),
        symbol: DEFAULT_LP_TOKEN_METADATA_SYMBOL.into(),
        ..Default::default()
    }
    .tlv_size_of()
}
