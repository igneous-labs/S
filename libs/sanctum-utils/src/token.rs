use solana_program::{
    instruction::Instruction, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
use solana_readonly_account::ReadonlyAccountData;
use spl_token::instruction::{close_account, transfer};

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

/// Sometimes you just want to read the supply of a mint account without caring about the other fields.
///
/// Should work with both token and token-2022
pub fn mint_supply<D: ReadonlyAccountData>(mint_account: D) -> Result<u64, ProgramError> {
    let spl_token::state::Mint { supply, .. } =
        spl_token::state::Mint::unpack(&mint_account.data())?;
    Ok(supply)
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

pub struct CloseTokenAccountKeys {
    pub token_program: Pubkey,
    pub account_to_close: Pubkey,
    pub refund_rent_to: Pubkey,
    pub authority: Pubkey,
}

impl CloseTokenAccountKeys {
    pub fn to_ix(&self) -> Result<Instruction, ProgramError> {
        close_account(
            &self.token_program,
            &self.account_to_close,
            &self.refund_rent_to,
            &self.authority,
            &[],
        )
    }
}
