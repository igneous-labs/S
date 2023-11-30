use solana_program::{
    instruction::Instruction, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
use solana_readonly_account::ReadonlyAccountData;
use spl_token::instruction::transfer;

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

pub struct TransferKeys {
    pub token_program: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub authority: Pubkey,
}

impl TransferKeys {
    pub fn to_ix(&self, amount: u64) -> Result<Instruction, ProgramError> {
        transfer(
            &self.token_program,
            &self.from,
            &self.to,
            &self.authority,
            &[],
            amount,
        )
    }
}
