use s_controller_interface::{LstState, SControllerError};
use sanctum_utils::associated_token::{
    create_ata_address, find_ata_address, CreateAtaAddressArgs, FindAtaAddressArgs,
};
use solana_program::pubkey::Pubkey;

use crate::program::{POOL_STATE_ID, PROTOCOL_FEE_ID};

pub fn create_pool_reserves_address(
    LstState {
        pool_reserves_bump,
        mint,
        ..
    }: &LstState,
    token_program: Pubkey,
) -> Result<Pubkey, SControllerError> {
    create_ata_address(CreateAtaAddressArgs {
        wallet: POOL_STATE_ID,
        mint: *mint,
        token_program,
        bump: *pool_reserves_bump,
    })
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
    create_ata_address(CreateAtaAddressArgs {
        wallet: PROTOCOL_FEE_ID,
        mint: *mint,
        token_program,
        bump: *protocol_fee_accumulator_bump,
    })
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
    find_ata_address(FindAtaAddressArgs {
        wallet: POOL_STATE_ID,
        mint: lst_mint,
        token_program,
    })
}

pub fn find_protocol_fee_accumulator_address(
    FindLstPdaAtaKeys {
        lst_mint,
        token_program,
    }: FindLstPdaAtaKeys,
) -> (Pubkey, u8) {
    find_ata_address(FindAtaAddressArgs {
        wallet: PROTOCOL_FEE_ID,
        mint: lst_mint,
        token_program,
    })
}
