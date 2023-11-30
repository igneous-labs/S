use solana_program::{
    account_info::AccountInfo,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
};
use spl_token::instruction::transfer;

pub struct TransferAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}

impl<'me, 'info> TransferAccounts<'me, 'info> {
    pub fn to_ix(&self, amount: u64) -> Result<Instruction, ProgramError> {
        transfer(
            self.token_program.key,
            self.from.key,
            self.to.key,
            self.authority.key,
            &[],
            amount,
        )
    }
}

/// TODO: handle token-2022
pub fn tranfer_tokens(accounts: TransferAccounts, amount: u64) -> Result<(), ProgramError> {
    let ix = accounts.to_ix(amount)?;
    invoke(
        &ix,
        &[
            accounts.from.clone(),
            accounts.to.clone(),
            accounts.authority.clone(),
        ],
    )
}

/// TODO: handle token-2022
pub fn transfer_tokens_signed(
    accounts: TransferAccounts,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let ix = accounts.to_ix(amount)?;
    invoke_signed(
        &ix,
        &[
            accounts.from.clone(),
            accounts.to.clone(),
            accounts.authority.clone(),
        ],
        signer_seeds,
    )
}
