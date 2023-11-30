use solana_program::pubkey::{Pubkey, PubkeyError};

use crate::program;

const FEE_ACCOUNT_SEED_PREFIX: &[u8] = b"fee";

pub struct FeeAccountFindPdaArgs {
    pub lst_mint: Pubkey,
}

impl FeeAccountFindPdaArgs {
    pub fn to_seed(&self) -> [&[u8]; 2] {
        [FEE_ACCOUNT_SEED_PREFIX, self.lst_mint.as_ref()]
    }

    pub fn get_fee_account_address_and_bump_seed(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.to_seed(), &program::ID)
    }
}

pub struct FeeAccountCreatePdaArgs {
    pub find: FeeAccountFindPdaArgs,
    pub bump: [u8; 1],
}

impl FeeAccountCreatePdaArgs {
    pub fn to_signer_seeds(&self) -> [&[u8]; 3] {
        let [prefix, lst_mint] = self.find.to_seed();
        [prefix, lst_mint, &self.bump]
    }

    pub fn get_fee_account_address(&self) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(self.to_signer_seeds().as_slice(), &program::ID)
    }
}
