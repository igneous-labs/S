use s_controller_interface::{LstState, SControllerError};
use sanctum_utils::associated_token::{create_ata_address, CreateAtaAddressArgs};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::ReadonlyAccountOwner;

use crate::program::STATE_ID;

pub fn create_pool_reserves_address<M: ReadonlyAccountOwner>(
    LstState {
        reserves_bump,
        mint,
        ..
    }: &LstState,
    lst_mint: M,
) -> Result<Pubkey, SControllerError> {
    create_ata_address(CreateAtaAddressArgs {
        wallet: STATE_ID,
        mint: *mint,
        token_program: *lst_mint.owner(),
        bump: *reserves_bump,
    })
    .map_err(|_e| SControllerError::InvalidReserves)
}
