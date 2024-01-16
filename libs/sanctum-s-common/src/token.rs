//! TODO: stuff in here should probably be moved to sanctum-token-lib

use sanctum_token_lib::mint_supply;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn verify_tokenkeg_or_22_mint(mint: &AccountInfo) -> Result<(), ProgramError> {
    if *mint.owner != spl_token::ID && *mint.owner != spl_token_2022::ID {
        return Err(ProgramError::IllegalOwner);
    }
    // TODO: change this to `sanctum_token_lib::ValidMintAccount::mint_is_initialized()`
    // when we upgrade `sanctum-token-lib`
    // trying to read mint.supply field verifies that the mint is initialized.
    mint_supply(mint)?;
    Ok(())
}
