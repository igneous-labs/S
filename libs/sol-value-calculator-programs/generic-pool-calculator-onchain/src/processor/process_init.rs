use generic_pool_calculator_interface::InitAccounts;
use generic_pool_calculator_lib::GenericPoolSolValCalc;
use solana_program::program_error::ProgramError;

pub fn process_init<P: GenericPoolSolValCalc>(_accounts: InitAccounts) -> Result<(), ProgramError> {
    todo!()
}
