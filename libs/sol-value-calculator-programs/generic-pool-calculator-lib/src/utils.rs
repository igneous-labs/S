use generic_pool_calculator_interface::GenericPoolCalculatorError;
use solana_program::{bpf_loader_upgradeable::UpgradeableLoaderState, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;

/// Attempts to deserialize a program account and read the
/// programdata_address contained within
pub fn read_programdata_addr<D: ReadonlyAccountData>(
    prog_acc: &D,
) -> Result<Pubkey, GenericPoolCalculatorError> {
    let prog_bytes = &prog_acc.data();
    let prog: UpgradeableLoaderState = bincode::deserialize(prog_bytes)
        .map_err(|_e| GenericPoolCalculatorError::InvalidStakePoolProgramData)?;
    if let UpgradeableLoaderState::Program {
        programdata_address,
    } = prog
    {
        Ok(programdata_address)
    } else {
        Err(GenericPoolCalculatorError::InvalidStakePoolProgramData)
    }
}
