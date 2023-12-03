use sanctum_utils::token::CloseTokenAccountKeys;
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

/// TODO: refactor to use sanctum_utils::TransferKeys
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

pub struct CloseTokenAccountAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub account_to_close: &'me AccountInfo<'info>,
    pub refund_rent_to: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}

impl From<&CloseTokenAccountAccounts<'_, '_>> for CloseTokenAccountKeys {
    fn from(
        CloseTokenAccountAccounts {
            token_program,
            account_to_close,
            refund_rent_to,
            authority,
        }: &CloseTokenAccountAccounts,
    ) -> Self {
        Self {
            token_program: *token_program.key,
            account_to_close: *account_to_close.key,
            refund_rent_to: *refund_rent_to.key,
            authority: *authority.key,
        }
    }
}

pub fn close_token_account_signed(
    accounts: CloseTokenAccountAccounts,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let keys: CloseTokenAccountKeys = (&accounts).into();
    let ix = keys.to_ix()?;
    invoke_signed(
        &ix,
        &[
            accounts.account_to_close.clone(),
            accounts.refund_rent_to.clone(),
            accounts.authority.clone(),
        ],
        signer_seeds,
    )
}
