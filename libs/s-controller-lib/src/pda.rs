use s_controller_interface::{LstState, SControllerError};
use sanctum_associated_token_lib::{CreateAtaAddressArgs, FindAtaAddressArgs};
use solana_program::pubkey::Pubkey;

use crate::{
    program::{POOL_STATE_ID, PROTOCOL_FEE_ID},
    DISABLE_POOL_AUTHORITY_LIST_PDA_SEED, LST_STATE_LIST_PDA_SEED, POOL_STATE_PDA_SEED,
    PROTOCOL_FEE_PDA_SEED, REBALANCE_RECORD_PDA_SEED,
};

pub fn create_pool_reserves_address(
    lst_state: &LstState,
    token_program: Pubkey,
) -> Result<Pubkey, SControllerError> {
    create_pool_reserves_address_with_pool_state_id(POOL_STATE_ID, lst_state, token_program)
}

pub fn create_pool_reserves_address_with_pool_state_id(
    pool_state_id: Pubkey,
    LstState {
        pool_reserves_bump,
        mint,
        ..
    }: &LstState,
    token_program: Pubkey,
) -> Result<Pubkey, SControllerError> {
    CreateAtaAddressArgs {
        find_ata_args: FindAtaAddressArgs {
            wallet: pool_state_id,
            mint: *mint,
            token_program,
        },
        bump: *pool_reserves_bump,
    }
    .create_ata_address()
    .map_err(|_e| SControllerError::InvalidReserves)
}

pub fn create_protocol_fee_accumulator_address(
    lst_state: &LstState,
    token_program: Pubkey,
) -> Result<Pubkey, SControllerError> {
    create_protocol_fee_accumulator_address_with_protocol_fee_id(
        PROTOCOL_FEE_ID,
        lst_state,
        token_program,
    )
}

pub fn create_protocol_fee_accumulator_address_with_protocol_fee_id(
    protocol_fee_id: Pubkey,
    LstState {
        protocol_fee_accumulator_bump,
        mint,
        ..
    }: &LstState,
    token_program: Pubkey,
) -> Result<Pubkey, SControllerError> {
    CreateAtaAddressArgs {
        find_ata_args: FindAtaAddressArgs {
            wallet: protocol_fee_id,
            mint: *mint,
            token_program,
        },
        bump: *protocol_fee_accumulator_bump,
    }
    .create_ata_address()
    .map_err(|_e| SControllerError::InvalidReserves)
}

#[derive(Clone, Copy, Debug)]
pub struct FindLstPdaAtaKeys {
    pub lst_mint: Pubkey,
    pub token_program: Pubkey,
}

pub fn find_pool_reserves_address(keys: FindLstPdaAtaKeys) -> (Pubkey, u8) {
    find_pool_reserves_address_with_pool_state_id(POOL_STATE_ID, keys)
}

pub fn find_pool_reserves_address_with_pool_state_id(
    pool_state_id: Pubkey,
    FindLstPdaAtaKeys {
        lst_mint,
        token_program,
    }: FindLstPdaAtaKeys,
) -> (Pubkey, u8) {
    FindAtaAddressArgs {
        wallet: pool_state_id,
        mint: lst_mint,
        token_program,
    }
    .find_ata_address()
}

pub fn find_protocol_fee_accumulator_address(keys: FindLstPdaAtaKeys) -> (Pubkey, u8) {
    find_protocol_fee_accumulator_address_with_protocol_fee_id(PROTOCOL_FEE_ID, keys)
}

pub fn find_protocol_fee_accumulator_address_with_protocol_fee_id(
    protocol_fee_id: Pubkey,
    FindLstPdaAtaKeys {
        lst_mint,
        token_program,
    }: FindLstPdaAtaKeys,
) -> (Pubkey, u8) {
    FindAtaAddressArgs {
        wallet: protocol_fee_id,
        mint: lst_mint,
        token_program,
    }
    .find_ata_address()
}

/// Finds the pool state PDA
/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::POOL_STATE_ID`] directly
pub fn find_pool_state_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[POOL_STATE_PDA_SEED], &program_id)
}

/// Finds the lst_state_list PDA
/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::LST_STATE_LIST_ID`] directly
pub fn find_lst_state_list_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[LST_STATE_LIST_PDA_SEED], &program_id)
}

/// Finds the disable pool authority list PDA
/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::DISABLE_POOL_AUTHORITY_LIST_ID`] directly
pub fn find_disable_pool_authority_list_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DISABLE_POOL_AUTHORITY_LIST_PDA_SEED], &program_id)
}

/// Finds the rebalance record PDA
/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::REBALANCE_RECORD_ID`] directly
pub fn find_rebalance_record_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[REBALANCE_RECORD_PDA_SEED], &program_id)
}

/// Finds the protocol fee auth PDA
/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::PROTOCOL_FEE_ID`] directly.
/// Returns the PDA that has authority over all the protocol fee accumulator token accounts
pub fn find_protocol_fee_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PROTOCOL_FEE_PDA_SEED], &program_id)
}
