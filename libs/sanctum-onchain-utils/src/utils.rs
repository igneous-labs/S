use solana_program::{
    account_info::AccountInfo, instruction::AccountMeta, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn load_accounts<'a, 'info, A, const LEN: usize>(
    accounts_slice: &'a [AccountInfo<'info>],
) -> Result<A, ProgramError>
where
    &'a [AccountInfo<'info>; LEN]: Into<A>,
{
    let subslice = accounts_slice
        .get(..LEN)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let accounts_arr: &[AccountInfo; LEN] = subslice.try_into().unwrap();
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

// idk why this isnt a util fn in solana-program
pub fn account_info_to_account_meta(
    AccountInfo {
        key,
        is_signer,
        is_writable,
        ..
    }: &AccountInfo,
) -> AccountMeta {
    AccountMeta {
        pubkey: **key,
        is_signer: *is_signer,
        is_writable: *is_writable,
    }
}
