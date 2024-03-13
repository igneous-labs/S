use solana_program::{
    bpf_loader_upgradeable,
    pubkey::{Pubkey, PubkeyError},
};

use crate::CALCULATOR_STATE_SEED;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalculatorStateFindPdaArgs {
    pub program_id: Pubkey,
}

impl CalculatorStateFindPdaArgs {
    pub const fn to_seed(&self) -> [&[u8]; 1] {
        [CALCULATOR_STATE_SEED]
    }

    pub fn get_calculator_state_address_and_bump_seed(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.to_seed(), &self.program_id)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalculatorStateCreatePdaArgs {
    pub find_pda_args: CalculatorStateFindPdaArgs,
    pub bump: u8,
}

impl CalculatorStateCreatePdaArgs {
    pub const fn to_signer_seeds(&self) -> [&[u8]; 2] {
        let [seed] = self.find_pda_args.to_seed();
        [seed, std::slice::from_ref(&self.bump)]
    }

    pub fn get_calculator_state_address(&self) -> Result<Pubkey, PubkeyError> {
        Pubkey::create_program_address(&self.to_signer_seeds(), &self.find_pda_args.program_id)
    }
}

pub struct ProgDataFindPdaArgs {
    pub program_id: Pubkey,
}

impl ProgDataFindPdaArgs {
    pub fn get_upgradeable_progdata_addr_and_bump(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[self.program_id.as_ref()], &bpf_loader_upgradeable::ID)
    }
}
