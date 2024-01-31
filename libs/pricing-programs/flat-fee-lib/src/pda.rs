use solana_program::pubkey::{Pubkey, PubkeyError};

use crate::program;

pub const FEE_ACCOUNT_SEED_PREFIX: &[u8] = b"fee";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramStateFindPdaArgs {
    pub program_id: Pubkey,
}

impl ProgramStateFindPdaArgs {
    pub const fn to_seed(&self) -> [&[u8]; 1] {
        [program::STATE_SEED]
    }

    pub fn get_program_state_address_and_bump_seed(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.to_seed(), &self.program_id)
    }
}

pub struct ProgramStateCreatePdaArgs {
    pub find_pda_args: ProgramStateFindPdaArgs,
    pub bump: u8,
}

impl ProgramStateCreatePdaArgs {
    pub const fn to_signer_seed(&self) -> [&[u8]; 2] {
        let [seed] = self.find_pda_args.to_seed();
        [seed, std::slice::from_ref(&self.bump)]
    }

    pub fn get_program_state_address(&self) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.to_signer_seed(), &self.find_pda_args.program_id)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeeAccountFindPdaArgs {
    pub program_id: Pubkey,
    pub lst_mint: Pubkey,
}

impl FeeAccountFindPdaArgs {
    pub fn to_seed(&self) -> [&[u8]; 2] {
        [FEE_ACCOUNT_SEED_PREFIX, self.lst_mint.as_ref()]
    }

    pub fn get_fee_account_address_and_bump_seed(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.to_seed(), &self.program_id)
    }
}

pub struct FeeAccountCreatePdaArgs {
    pub find_pda_args: FeeAccountFindPdaArgs,
    pub bump: u8,
}

impl FeeAccountCreatePdaArgs {
    pub fn to_signer_seeds(&self) -> [&[u8]; 3] {
        let [prefix, lst_mint] = self.find_pda_args.to_seed();

        [prefix, lst_mint, std::slice::from_ref(&self.bump)]
    }

    pub fn get_fee_account_address(&self) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.to_signer_seeds(), &self.find_pda_args.program_id)
    }
}
