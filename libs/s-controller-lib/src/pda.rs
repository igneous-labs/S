use s_controller_interface::{LstState, SControllerError};
use sanctum_utils::associated_token::{create_ata_address, CreateAtaAddressArgs};
use solana_program::pubkey::Pubkey;

use crate::program::STATE_ID;

pub fn create_pool_reserves_address(
    LstState {
        reserves_bump,
        mint,
        ..
    }: &LstState,
    token_program: Pubkey,
) -> Result<Pubkey, SControllerError> {
    create_ata_address(CreateAtaAddressArgs {
        wallet: STATE_ID,
        mint: *mint,
        token_program,
        bump: *reserves_bump,
    })
    .map_err(|_e| SControllerError::InvalidReserves)
}
