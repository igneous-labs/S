use generic_pool_calculator_lib::GenericPoolSolValCalc;
use marinade_keys::{marinade_program, marinade_program_progdata};
use solana_program::pubkey::Pubkey;

mod calc;
mod instructions;

pub use account_resolvers::*;
pub use calc::*;
pub use instructions::*;

pub mod program {
    sanctum_macros::declare_program_keys!(
        "mare3SCyfZkAndpBRBeonETmkCCB3TJTTrz8ZN2dnhP",
        [("marinade_calculator_state", b"state")]
    );
}

/// TODO: set actual initial manager
pub mod initial_manager {
    sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);
}

pub struct MarinadeSolValCalc;

impl GenericPoolSolValCalc for MarinadeSolValCalc {
    const POOL_PROGRAM_ID: Pubkey = marinade_program::ID;
    const POOL_PROGRAM_PROGDATA_ID: Pubkey = marinade_program_progdata::ID;
    const CALCULATOR_STATE_PDA: Pubkey = program::MARINADE_CALCULATOR_STATE_ID;
    const CALCULATOR_STATE_BUMP: u8 = program::MARINADE_CALCULATOR_STATE_BUMP;
    const ID: Pubkey = program::ID;
}

mod account_resolvers {
    use generic_pool_calculator_lib::account_resolvers::LstSolCommonIntermediateKeys;
    use marinade_keys::{marinade_state, msol};

    pub const MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS: LstSolCommonIntermediateKeys =
        LstSolCommonIntermediateKeys {
            lst_mint: msol::ID,
            pool_state: marinade_state::ID,
        };
}
