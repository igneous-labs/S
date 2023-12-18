use sanctum_token_ratio::U64ValueRange;
use solana_program::program_error::ProgramError;

pub trait SolValueCalculator {
    fn calc_lst_to_sol(&self, lst_amount: u64) -> Result<U64ValueRange, ProgramError>;
    fn calc_sol_to_lst(&self, lamports_amount: u64) -> Result<U64ValueRange, ProgramError>;
}
