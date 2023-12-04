pub mod account_resolvers;
pub mod calc;
pub mod pda;
pub mod utils;

pub mod program {
    pub const STATE_SIZE: usize = 34;
    static_assertions::const_assert_eq!(
        std::mem::size_of::<flat_fee_interface::ProgramState>(),
        STATE_SIZE,
    );

    sanctum_macros::declare_program_keys!(
        "TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111",
        [("state", b"state")]
    );
}

// TODO: should these be in onchain program instead of being in the lib?
pub mod initial_constants {
    pub mod initial_manager {
        sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);
    }

    pub const INITIAL_LP_WITHDRAWAL_FEE_BPS: u16 = 5;
}

// TODO: move this
const BPS_DENOM: u16 = 10_000;
