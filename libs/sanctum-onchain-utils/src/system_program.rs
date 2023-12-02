use solana_program::{
    account_info::AccountInfo,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{self, allocate, assign},
    system_program,
    sysvar::Sysvar,
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
    /// defaults to rent exempt amount for space if not provided
    pub lamports: Option<u64>,
}

/// Run the CreateAccount SystemInstruction for an externally signed account.
/// system_program AccountInfo must be in scope
pub fn create_blank_account(
    CreateAccountAccounts { from, to }: CreateAccountAccounts,
    CreateAccountArgs {
        space,
        owner,
        lamports,
    }: CreateAccountArgs,
) -> Result<(), ProgramError> {
    let space_u64: u64 = space
        .try_into()
        .map_err(|_e| ProgramError::InvalidArgument)?;
    let lamports = lamports.map_or_else(
        || Ok::<u64, ProgramError>(Rent::get()?.minimum_balance(space)),
        Ok,
    )?;
    let ix = system_instruction::create_account(from.key, to.key, lamports, space_u64, &owner);
    invoke(&ix, &[from.clone(), to.clone()])
}

/// Run the CreateAccount SystemInstruction for a PDA
/// system_program AccountInfo must be in scope
pub fn create_pda(
    CreateAccountAccounts { from, to }: CreateAccountAccounts,
    CreateAccountArgs {
        space,
        owner,
        lamports,
    }: CreateAccountArgs,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let space_u64: u64 = space
        .try_into()
        .map_err(|_e| ProgramError::InvalidArgument)?;
    let lamports = lamports.map_or_else(
        || Ok::<u64, ProgramError>(Rent::get()?.minimum_balance(space)),
        Ok,
    )?;
    let ix = system_instruction::create_account(from.key, to.key, lamports, space_u64, &owner);
    invoke_signed(&ix, &[from.clone(), to.clone()], signer_seeds)
}

#[derive(Clone, Copy, Debug)]
pub struct CloseAccountAccounts<'me, 'info> {
    pub refund_rent_to: &'me AccountInfo<'info>,
    pub close: &'me AccountInfo<'info>,
}

pub fn close_account(
    CloseAccountAccounts {
        refund_rent_to,
        close,
    }: CloseAccountAccounts,
) -> Result<(), ProgramError> {
    transfer_direct_increment(
        TransferAccounts {
            from: close,
            to: refund_rent_to,
        },
        close.lamports(),
    )?;
    close.assign(&system_program::ID);
    close.realloc(0, false)
}

pub fn allocate_pda(
    pda: &AccountInfo,
    space: usize,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let ix = allocate(
        pda.key,
        space
            .try_into()
            .map_err(|_e| ProgramError::InvalidArgument)?,
    );
    invoke_signed(&ix, &[pda.clone()], signer_seeds)
}

pub fn assign_pda(
    pda: &AccountInfo,
    owner: Pubkey,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let ix = assign(pda.key, &owner);
    invoke_signed(&ix, &[pda.clone()], signer_seeds)
}

#[derive(Clone, Copy, Debug)]
pub struct TransferAccounts<'me, 'info> {
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
}

pub fn transfer(
    TransferAccounts { from, to }: TransferAccounts,
    lamports: u64,
) -> Result<(), ProgramError> {
    let ix = system_instruction::transfer(from.key, to.key, lamports);
    invoke(&ix, &[from.clone(), to.clone()])
}

/// Transfer by directly decrementing one account's lamports and
/// incrementing another's
pub fn transfer_direct_increment(
    TransferAccounts { from, to }: TransferAccounts,
    lamports: u64,
) -> Result<(), ProgramError> {
    let to_starting_lamports = to.lamports();
    **to.try_borrow_mut_lamports()? = to_starting_lamports
        .checked_add(lamports)
        .ok_or(ProgramError::InvalidArgument)?;
    let from_starting_lamports = from.lamports();
    **from.try_borrow_mut_lamports()? = from_starting_lamports
        .checked_sub(lamports)
        .ok_or(ProgramError::InvalidArgument)?;
    Ok(())
}
