use s_controller_interface::DisablePoolKeys;
use solana_program::pubkey::Pubkey;

use crate::{
    find_disable_pool_authority_list_address, find_pool_state_address,
    program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID},
};

#[derive(Clone, Copy, Debug)]
pub struct DisablePoolFreeArgs {
    pub signer: Pubkey,
}

impl DisablePoolFreeArgs {
    pub fn resolve(&self) -> DisablePoolKeys {
        self.resolve_inner(POOL_STATE_ID, DISABLE_POOL_AUTHORITY_LIST_ID)
    }

    pub fn resolve_for_prog(&self, program_id: Pubkey) -> DisablePoolKeys {
        let pool_state_id = find_pool_state_address(program_id).0;
        let disable_pool_authority_list_id = find_disable_pool_authority_list_address(program_id).0;
        self.resolve_inner(pool_state_id, disable_pool_authority_list_id)
    }

    fn resolve_inner(
        &self,
        pool_state_id: Pubkey,
        disable_pool_authority_list_id: Pubkey,
    ) -> DisablePoolKeys {
        DisablePoolKeys {
            signer: self.signer,
            pool_state: pool_state_id,
            disable_pool_authority_list: disable_pool_authority_list_id,
        }
    }
}
