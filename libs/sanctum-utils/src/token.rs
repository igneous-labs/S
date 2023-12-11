use solana_program::{instruction::Instruction, program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use spl_token::instruction::{close_account, transfer};
use spl_token_2022::extension::StateWithExtensions;

/// Sometimes you just want to read the balance of a token account without caring about the other fields.
/// This works for both token-2022 and tokenkeg accounts.
/// TODO: switch to GenericTokenAccount to save compute
pub fn token_account_balance<D: ReadonlyAccountData>(
    token_account: D,
) -> Result<u64, ProgramError> {
    let data = token_account.data();
    let state = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&data)?;
    Ok(state.base.amount)
}

/// Sometimes you just want to read the supply of a mint account without caring about the other fields.
/// This works for both token-2022 and tokenkeg mints
pub fn mint_supply<D: ReadonlyAccountData>(mint_account: D) -> Result<u64, ProgramError> {
    let data = mint_account.data();
    let state = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&data)?;
    Ok(state.base.supply)
}

/// Sometimes you just want to read the mint of a token account without caring about the other fields.
/// This works for both token-2022 and tokenkeg accounts
/// TODO: switch to GenericTokenAccount to save compute
pub fn token_account_mint<D: ReadonlyAccountData>(
    token_account: D,
) -> Result<Pubkey, ProgramError> {
    let data = token_account.data();
    let state = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&data)?;
    Ok(state.base.mint)
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
