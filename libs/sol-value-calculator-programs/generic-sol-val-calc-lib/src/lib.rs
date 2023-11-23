use solana_program::pubkey::Pubkey;

mod utils;

pub mod account_resolvers;

/// Implement this trait for individual generic pool SOL value calculator programs
pub trait GenericPoolSolValCalc {
    /// Program ID of the stake pool program that the calculator program works for
    const POOL_PROGRAM_ID: Pubkey;

    /// CalculatorState of the calculator program located at PDA ["state"]
    const CALCULATOR_STATE_PDA: Pubkey;
}
