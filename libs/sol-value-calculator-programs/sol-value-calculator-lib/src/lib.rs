use solana_program::program_error::ProgramError;

pub trait SolValueCalculator {
    fn calc_lst_to_sol(&self, lst_amount: u64) -> Result<u64, ProgramError>;
    fn calc_sol_to_lst(&self, lamports_amount: u64) -> Result<u64, ProgramError>;
}
