use sanctum_utils::token::CloseTokenAccountKeys;
use solana_program::{
    account_info::AccountInfo,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction::{transfer, AuthorityType};

pub struct TransferTokensAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}

impl<'me, 'info> TransferTokensAccounts<'me, 'info> {
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

/// TODO: handle token-2022 transfer hook?
pub fn transfer_tokens(accounts: TransferTokensAccounts, amount: u64) -> Result<(), ProgramError> {
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

/// TODO: handle token-2022 transfer hook?
pub fn transfer_tokens_signed(
    accounts: TransferTokensAccounts,
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

#[derive(Clone, Copy, Debug)]
pub struct SetAuthorityAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub to_change: &'me AccountInfo<'info>,
    pub current_authority: &'me AccountInfo<'info>,
}

pub fn set_authority(
    SetAuthorityAccounts {
        token_program,
        to_change,
        current_authority,
    }: SetAuthorityAccounts,
    authority_type: AuthorityType,
    new_authority: Option<Pubkey>,
) -> Result<(), ProgramError> {
    let ix = spl_token::instruction::set_authority(
        token_program.key,
        to_change.key,
        new_authority.as_ref(),
        authority_type,
        current_authority.key,
        &[],
    )?;
    invoke(&ix, &[to_change.clone(), current_authority.clone()])
}

pub struct MintToAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub mint_to: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

pub fn mint_to_signed(
    MintToAccounts {
        mint,
        mint_to,
        mint_authority,
        token_program,
    }: MintToAccounts,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let ix = spl_token_2022::instruction::mint_to(
        token_program.key,
        mint.key,
        mint_to.key,
        mint_authority.key,
        &[],
        amount,
    )?;
    invoke_signed(
        &ix,
        &[mint.clone(), mint_to.clone(), mint_authority.clone()],
        signer_seeds,
    )
}

pub struct BurnTokensAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub burn_from: &'me AccountInfo<'info>,
    pub burn_from_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}

pub fn burn_tokens(
    BurnTokensAccounts {
        mint,
        burn_from,
        burn_from_authority,
        token_program,
    }: BurnTokensAccounts,
    amount: u64,
) -> Result<(), ProgramError> {
    let ix = spl_token_2022::instruction::burn(
        token_program.key,
        burn_from.key,
        mint.key,
        burn_from_authority.key,
        &[],
        amount,
    )?;
    invoke(
        &ix,
        &[burn_from.clone(), mint.clone(), burn_from_authority.clone()],
    )
}
