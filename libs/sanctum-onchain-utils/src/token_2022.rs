use solana_program::{
    account_info::AccountInfo,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_metadata_interface::instruction::Initialize;

#[derive(Clone, Copy, Debug)]
pub struct InitializeMint2Args {
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
}

pub fn initialize_mint2(
    mint: &AccountInfo,
    InitializeMint2Args {
        decimals,
        mint_authority,
        freeze_authority,
    }: InitializeMint2Args,
) -> Result<(), ProgramError> {
    let ix = spl_token_2022::instruction::initialize_mint2(
        &spl_token_2022::ID,
        mint.key,
        &mint_authority,
        freeze_authority.as_ref(),
        decimals,
    )?;
    invoke(&ix, &[mint.clone()])
}

pub struct InitializeTransferFeeConfigArgs {
    pub transfer_fee_config_authority: Option<Pubkey>,
    pub withdraw_withheld_authority: Option<Pubkey>,
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

pub fn initialize_transfer_fee_config(
    mint: &AccountInfo,
    InitializeTransferFeeConfigArgs {
        transfer_fee_config_authority,
        withdraw_withheld_authority,
        transfer_fee_basis_points,
        maximum_fee,
    }: InitializeTransferFeeConfigArgs,
) -> Result<(), ProgramError> {
    let ix = spl_token_2022::extension::transfer_fee::instruction::initialize_transfer_fee_config(
        &spl_token_2022::ID,
        mint.key,
        transfer_fee_config_authority.as_ref(),
        withdraw_withheld_authority.as_ref(),
        transfer_fee_basis_points,
        maximum_fee,
    )?;
    invoke(&ix, &[mint.clone()])
}

pub struct InitializeMintTokenMetadataAccounts<'me, 'info> {
    pub mint: &'me AccountInfo<'info>,
    pub update_authority: &'me AccountInfo<'info>,
    pub mint_authority: &'me AccountInfo<'info>,
}

pub struct InitializeMintTokenMetadataArgs {
    pub initial_metadata: Initialize,
}

/// Initialize token metadata stored in mint with TokenMetadata extension
pub fn initialize_mint_token_metadata_signed(
    InitializeMintTokenMetadataAccounts {
        mint,
        update_authority,
        mint_authority,
    }: InitializeMintTokenMetadataAccounts,
    Initialize { name, symbol, uri }: Initialize,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let ix = spl_token_metadata_interface::instruction::initialize(
        &spl_token_2022::ID,
        mint.key,
        update_authority.key,
        mint.key,
        mint_authority.key,
        name,
        symbol,
        uri,
    );
    invoke_signed(
        &ix,
        &[
            mint.clone(),
            update_authority.clone(),
            mint.clone(),
            mint_authority.clone(),
        ],
        signer_seeds,
    )
}

pub struct InitializeMetadataPointerArgs {
    pub authority: Option<Pubkey>,
    pub metadata_address: Option<Pubkey>,
}

pub fn initialize_metadata_pointer(
    mint: &AccountInfo,
    InitializeMetadataPointerArgs {
        authority,
        metadata_address,
    }: InitializeMetadataPointerArgs,
) -> Result<(), ProgramError> {
    let ix = spl_token_2022::extension::metadata_pointer::instruction::initialize(
        &spl_token_2022::ID,
        mint.key,
        authority,
        metadata_address,
    )?;
    invoke(&ix, &[mint.clone()])
}
