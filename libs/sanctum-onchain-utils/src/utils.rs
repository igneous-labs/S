use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

pub fn load_accounts<'a, 'info, A, const LEN: usize>(
    accounts_slice: &'a [AccountInfo<'info>],
) -> Result<A, ProgramError>
where
    &'a [AccountInfo<'info>; LEN]: Into<A>,
{
    let accounts_arr: &[AccountInfo; LEN] = accounts_slice
        .try_into()
        .map_err(|_e| ProgramError::NotEnoughAccountKeys)?;
    Ok(accounts_arr.into())
}

pub fn log_and_return_wrong_acc_err((actual, expected): (Pubkey, Pubkey)) -> ProgramError {
    msg!("Wrong account. Expected: {}, Got: {}", expected, actual);
    ProgramError::InvalidArgument
}

pub fn log_and_return_acc_privilege_err((info, err): (&AccountInfo, ProgramError)) -> ProgramError {
    msg!("Writable/signer privilege escalated for: {}", info.key);
    err
}
