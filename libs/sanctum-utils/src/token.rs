use solana_program::{program_error::ProgramError, program_pack::Pack};
use solana_readonly_account::ReadonlyAccountData;

/// Sometimes you just want to read the balance of a token account without caring about the other fields.
///
/// Should work with both token and token-2022
pub fn token_account_balance<D: ReadonlyAccountData>(
    token_account: D,
) -> Result<u64, ProgramError> {
    let spl_token::state::Account { amount, .. } =
        spl_token::state::Account::unpack(&token_account.data())?;
    Ok(amount)
}
