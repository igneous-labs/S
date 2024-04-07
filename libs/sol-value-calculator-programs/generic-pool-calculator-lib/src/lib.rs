use generic_pool_calculator_interface::CalculatorState;
use solana_program::pubkey::Pubkey;
use static_assertions::const_assert_eq;

mod lst_sol_common;

pub mod account_resolvers;
pub mod pda;
pub mod utils;

pub use lst_sol_common::*;

pub const CALCULATOR_STATE_SEED: &[u8] = b"state";

// std::mem::size_of is a const fn so we dont technically need this
// but this assert helps guard against unexpected size changes
pub const CALCULATOR_STATE_SIZE: usize = 40;
const_assert_eq!(
    std::mem::size_of::<CalculatorState>(),
    CALCULATOR_STATE_SIZE
);

/// Implement this trait for individual generic pool SOL value calculator programs
pub trait GenericPoolSolValCalc {
    /// Program ID of the stake pool program that the calculator program works for
    const POOL_PROGRAM_ID: Pubkey;

    // Address of the stake pool program's executable data account
    const POOL_PROGRAM_PROGDATA_ID: Pubkey;

    /// CalculatorState of the calculator program located at PDA ["state"]
    const CALCULATOR_STATE_PDA: Pubkey;

    /// Bump seed of CALCULATOR_STATE_PDA
    const CALCULATOR_STATE_BUMP: u8;

    /// The SOL value calculator program ID
    const ID: Pubkey;
}
