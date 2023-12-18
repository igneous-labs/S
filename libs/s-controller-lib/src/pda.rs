use s_controller_interface::{LstState, SControllerError};
use sanctum_associated_token_lib::{CreateAtaAddressArgs, FindAtaAddressArgs};
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
