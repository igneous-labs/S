use solana_program::pubkey::{Pubkey, PubkeyError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateAtaAddressArgs {
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub bump: u8,
}

/// spl-associated-token-account only exports the find_program_address version.
/// This uses create_program_address with a known bump to reduce compute used
pub fn create_ata_address(
    CreateAtaAddressArgs {
        wallet,
        mint,
        token_program,
        bump,
    }: CreateAtaAddressArgs,
) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_program_address(
        &[
            &wallet.to_bytes(),
            &token_program.to_bytes(),
            &mint.to_bytes(),
            &[bump],
        ],
        &spl_associated_token_account::ID,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FindAtaAddressArgs {
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}

/// spl-associated-token-account doesnt export a find_program_address
/// that also returns the found bump
pub fn find_ata_address(
    FindAtaAddressArgs {
        wallet,
        mint,
        token_program,
    }: FindAtaAddressArgs,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &wallet.to_bytes(),
            &token_program.to_bytes(),
            &mint.to_bytes(),
        ],
        &spl_associated_token_account::ID,
    )
}
