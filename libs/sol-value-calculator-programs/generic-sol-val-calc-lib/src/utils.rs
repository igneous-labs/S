use solana_program::{
    bpf_loader_upgradeable::UpgradeableLoaderState, program_error::ProgramError, pubkey::Pubkey,
};
use solana_readonly_account::ReadonlyAccountData;

/// Attempts to deserialize a program account and read the
/// programdata_address contained within
pub fn read_programdata_addr<D: ReadonlyAccountData>(prog_acc: &D) -> Result<Pubkey, ProgramError> {
    let prog_bytes = &prog_acc.data();
    let prog: UpgradeableLoaderState =
        bincode::deserialize(prog_bytes).map_err(|_e| ProgramError::InvalidAccountData)?;
    if let UpgradeableLoaderState::Program {
        programdata_address,
    } = prog
    {
        Ok(programdata_address)
    } else {
        Err(ProgramError::InvalidAccountData)
    }
}
