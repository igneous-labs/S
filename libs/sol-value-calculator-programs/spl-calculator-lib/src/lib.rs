use generic_pool_calculator_lib::GenericPoolSolValCalc;
use solana_program::pubkey::Pubkey;
use spl_stake_pool_keys::{spl_stake_pool_program, spl_stake_pool_program_progdata};

mod account_resolvers;
mod calc;
mod instructions;

pub use account_resolvers::*;
pub use calc::*;
pub use instructions::*;

pub mod program {
    sanctum_macros::declare_program_keys!(
        "sp1V4h2gWorkGhVcazBc22Hfo2f5sd7jcjT4EDPrWFF",
        [("spl_calculator_state", b"state")]
    );
}

/// TODO: set actual initial manager
pub mod initial_manager {
    sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);
}

pub struct SplSolValCalc;

impl GenericPoolSolValCalc for SplSolValCalc {
    const POOL_PROGRAM_ID: Pubkey = spl_stake_pool_program::ID;
    const POOL_PROGRAM_PROGDATA_ID: Pubkey = spl_stake_pool_program_progdata::ID;
    const CALCULATOR_STATE_PDA: Pubkey = program::SPL_CALCULATOR_STATE_ID;
    const CALCULATOR_STATE_BUMP: u8 = program::SPL_CALCULATOR_STATE_BUMP;
    const ID: Pubkey = program::ID;
}
