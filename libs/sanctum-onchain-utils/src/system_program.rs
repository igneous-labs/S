use solana_program::{
    account_info::AccountInfo, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey,
    rent::Rent, system_instruction::create_account, sysvar::Sysvar,
};

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountAccounts<'me, 'info> {
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountArgs {
    pub space: usize,
    pub owner: Pubkey,
}

/// Run the CreateAccount SystemInstruction for a PDA
/// system_program AccountInfo must be in scope
pub fn create_pda(
    CreateAccountAccounts { from, to }: CreateAccountAccounts,
    CreateAccountArgs { space, owner }: CreateAccountArgs,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let rent = Rent::get()?;
    let space_u64: u64 = space
        .try_into()
        .map_err(|_e| ProgramError::InvalidArgument)?;
    let lamports = rent.minimum_balance(space);
    let ix = create_account(from.key, to.key, lamports, space_u64, &owner);
    invoke_signed(&ix, &[from.clone(), to.clone()], signer_seeds)
}
