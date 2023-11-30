pub mod account_resolvers;
pub mod pda;
pub mod processor;
mod utils;

pub mod program {
    sanctum_macros::declare_program_keys!(
        "TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111",
        [("state", b"state")]
    );
}

pub mod initial_manager {
    sanctum_macros::declare_program_keys!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111", []);
}
