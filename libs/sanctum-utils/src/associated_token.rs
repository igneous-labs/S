use solana_program::pubkey::{Pubkey, PubkeyError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateAtaAddressArgs {
    pub find_ata_args: FindAtaAddressArgs,
    pub bump: [u8; 1],
}

impl CreateAtaAddressArgs {
    pub fn to_signer_seeds(&self) -> [&[u8]; 4] {
        let [wallet, token_program, mint] = self.find_ata_args.to_seeds();
        [wallet, token_program, mint, &self.bump]
    }

    pub fn create_ata_address(&self) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.to_signer_seeds(), &spl_associated_token_account::ID)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FindAtaAddressArgs {
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
}

impl FindAtaAddressArgs {
    pub fn to_seeds(&self) -> [&[u8]; 3] {
        [
            self.wallet.as_ref(),
            self.token_program.as_ref(),
            self.mint.as_ref(),
        ]
    }

    /// spl-associated-token-account doesnt export a find_program_address
    /// that also returns the found bump
    pub fn find_ata_address(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.to_seeds(), &spl_associated_token_account::ID)
    }
}
