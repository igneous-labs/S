use solana_program::{
    account_info::AccountInfo, program::invoke, program_error::ProgramError, pubkey::Pubkey,
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

pub struct InitializeMintTokenMetadataArgs {
    pub initial_metadata: Initialize,
    pub update_authority: Pubkey,
    pub mint_authority: Pubkey,
}

/// Initialize token metadata stored in mint with TokenMetadata extension
pub fn initialize_mint_token_metadata(
    mint: &AccountInfo,
    InitializeMintTokenMetadataArgs {
        initial_metadata: Initialize { name, symbol, uri },
        update_authority,
        mint_authority,
    }: InitializeMintTokenMetadataArgs,
) -> Result<(), ProgramError> {
    let ix = spl_token_metadata_interface::instruction::initialize(
        &spl_token_2022::ID,
        mint.key,
        &update_authority,
        mint.key,
        &mint_authority,
        name,
        symbol,
        uri,
    );
    invoke(&ix, &[mint.clone()])
}
