use s_controller_interface::{LstState, SControllerError};
use sanctum_associated_token_lib::{CreateAtaAddressArgs, FindAtaAddressArgs};
use solana_program::pubkey::Pubkey;

use crate::{
    program::{POOL_STATE_ID, PROTOCOL_FEE_ID},
    DISABLE_POOL_AUTHORITY_LIST_PDA_SEED, LST_STATE_LIST_PDA_SEED, POOL_STATE_PDA_SEED,
    PROTOCOL_FEE_PDA_SEED, REBALANCE_RECORD_PDA_SEED,
};

pub fn create_pool_reserves_address(
    LstState {
        pool_reserves_bump,
        mint,
        ..
    }: &LstState,
    token_program: Pubkey,
) -> Result<Pubkey, SControllerError> {
    CreateAtaAddressArgs {
        find_ata_args: FindAtaAddressArgs {
            wallet: POOL_STATE_ID,
            mint: *mint,
            token_program,
        },
        bump: *pool_reserves_bump,
    }
    .create_ata_address()
    .map_err(|_e| SControllerError::InvalidReserves)
}

pub fn create_protocol_fee_accumulator_address(
    LstState {
        protocol_fee_accumulator_bump,
        mint,
        ..
    }: &LstState,
    token_program: Pubkey,
) -> Result<Pubkey, SControllerError> {
    CreateAtaAddressArgs {
        find_ata_args: FindAtaAddressArgs {
            wallet: PROTOCOL_FEE_ID,
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

pub fn find_pool_reserves_address(
    FindLstPdaAtaKeys {
        lst_mint,
        token_program,
    }: FindLstPdaAtaKeys,
) -> (Pubkey, u8) {
    FindAtaAddressArgs {
        wallet: POOL_STATE_ID,
        mint: lst_mint,
        token_program,
    }
    .find_ata_address()
}

pub fn find_protocol_fee_accumulator_address(
    FindLstPdaAtaKeys {
        lst_mint,
        token_program,
    }: FindLstPdaAtaKeys,
) -> (Pubkey, u8) {
    FindAtaAddressArgs {
        wallet: PROTOCOL_FEE_ID,
        mint: lst_mint,
        token_program,
    }
    .find_ata_address()
}

/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::POOL_STATE_ID`] directly
pub fn find_pool_state_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[POOL_STATE_PDA_SEED], &program_id)
}

/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::LST_STATE_LIST_ID`] directly
pub fn find_lst_state_list_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[LST_STATE_LIST_PDA_SEED], &program_id)
}

/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::DISABLE_POOL_AUTHORITY_LIST_ID`] directly
pub fn find_disable_pool_authority_list_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DISABLE_POOL_AUTHORITY_LIST_PDA_SEED], &program_id)
}

/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::REBALANCE_RECORD_ID`] directly
pub fn find_rebalance_record_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[REBALANCE_RECORD_PDA_SEED], &program_id)
}

/// For dynamic program IDs.
/// If using crate's program ID, you can use [`crate::program::PROTOCOL_FEE_ID`] directly.
/// Returns the PDA that has authority over all the protocol fee accumulator token accounts
pub fn find_protocol_fee_address(program_id: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PROTOCOL_FEE_PDA_SEED], &program_id)
}
