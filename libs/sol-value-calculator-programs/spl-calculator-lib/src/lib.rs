use generic_pool_calculator_lib::GenericPoolSolValCalc;
use solana_program::pubkey::Pubkey;

pub mod account_resolvers;

pub mod program {
    sanctum_macros::declare_program_keys!(
        "sp1V4h2gWorkGhVcazBc22Hfo2f5sd7jcjT4EDPrWFF",
        [("spl_calculator_state", b"state")]
    );
}

mod spl_stake_pool_program {
    sanctum_macros::declare_program_keys!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy", []);
}

pub struct SplSolValCalc;

impl GenericPoolSolValCalc for SplSolValCalc {
    const POOL_PROGRAM_ID: Pubkey = spl_stake_pool_program::ID;
    const CALCULATOR_STATE_PDA: Pubkey = program::SPL_CALCULATOR_STATE_ID;
    const CALCULATOR_STATE_BUMP: u8 = program::SPL_CALCULATOR_STATE_BUMP;
}
