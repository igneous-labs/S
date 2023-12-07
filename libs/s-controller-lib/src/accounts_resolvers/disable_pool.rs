use s_controller_interface::DisablePoolKeys;
use solana_program::pubkey::Pubkey;

use crate::program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID};

#[derive(Clone, Copy, Debug)]
pub struct DisablePoolFreeArgs {
    pub signer: Pubkey,
}

impl DisablePoolFreeArgs {
    pub fn resolve(&self) -> DisablePoolKeys {
        DisablePoolKeys {
            signer: self.signer,
            pool_state: POOL_STATE_ID,
            disable_pool_authority_list: DISABLE_POOL_AUTHORITY_LIST_ID,
        }
    }
}
