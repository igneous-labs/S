use generic_pool_calculator_lib::GenericPoolSolValCalc;
use lido_keys::{lido_program, lido_program_progdata};
use solana_program::pubkey::Pubkey;

mod calc;
mod instructions;

pub use account_resolvers::*;
pub use calc::*;
pub use instructions::*;

pub mod program {
    sanctum_macros::declare_program_keys!(
        "1idUSy4MGGKyKhvjSnGZ6Zc7Q4eKQcibym4BkEEw9KR",
        [("lido_calculator_state", b"state")]
    );
}

/// TODO: set actual initial manager
pub mod initial_manager {
    sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);
}

pub struct SplSolValCalc;

impl GenericPoolSolValCalc for SplSolValCalc {
    const POOL_PROGRAM_ID: Pubkey = lido_program::ID;
    const POOL_PROGRAM_PROGDATA_ID: Pubkey = lido_program_progdata::ID;
    const CALCULATOR_STATE_PDA: Pubkey = program::LIDO_CALCULATOR_STATE_ID;
    const CALCULATOR_STATE_BUMP: u8 = program::LIDO_CALCULATOR_STATE_BUMP;
    const ID: Pubkey = program::ID;
}

mod account_resolvers {
    use generic_pool_calculator_lib::account_resolvers::LstSolCommonIntermediateKeys;
    use lido_keys::{lido_state, stsol};

    pub const LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS: LstSolCommonIntermediateKeys =
        LstSolCommonIntermediateKeys {
            lst_mint: stsol::ID,
            pool_state: lido_state::ID,
        };
}
