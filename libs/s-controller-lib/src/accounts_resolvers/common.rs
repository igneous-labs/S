use solana_program::pubkey::Pubkey;

use crate::{find_lst_state_list_address, find_pool_state_address, find_protocol_fee_address};

#[derive(Clone, Copy, Debug)]
pub struct SwapLiquidityPdas {
    pub pool_state: Pubkey,
    pub lst_state_list: Pubkey,
    pub protocol_fee: Pubkey,
}

impl SwapLiquidityPdas {
    pub fn find_for_program_id(program_id: Pubkey) -> Self {
        Self {
            pool_state: find_pool_state_address(program_id).0,
            lst_state_list: find_lst_state_list_address(program_id).0,
            protocol_fee: find_protocol_fee_address(program_id).0,
        }
    }
}
