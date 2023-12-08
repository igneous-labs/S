pub mod account_resolvers;
pub mod calc;
pub mod fee_bound;
pub mod pda;
pub mod utils;

pub mod program {
    pub const STATE_SIZE: usize = 34;
    pub const FEE_ACCOUNT_SIZE: usize = 6;

    static_assertions::const_assert_eq!(
        std::mem::size_of::<flat_fee_interface::ProgramState>(),
        STATE_SIZE,
    );

    static_assertions::const_assert_eq!(
        std::mem::size_of::<flat_fee_interface::FeeAccount>(),
        FEE_ACCOUNT_SIZE,
    );

    sanctum_macros::declare_program_keys!(
        "TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111",
        [("state", b"state")]
    );
}

pub mod initial_constants {
    pub mod initial_manager {
        sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);
    }

    pub const INITIAL_LP_WITHDRAWAL_FEE_BPS: u16 = 5;
}
