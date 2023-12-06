use s_controller_interface::SetAdminKeys;
use solana_program::pubkey::Pubkey;

use crate::{initial_authority, program::POOL_STATE_ID};

#[derive(Clone, Copy, Debug)]
pub struct SetAdminFreeArgs {
    pub new_admin: Pubkey,
}

impl SetAdminFreeArgs {
    pub fn resolve(self) -> SetAdminKeys {
        let SetAdminFreeArgs { new_admin } = self;
        SetAdminKeys {
            current_admin: initial_authority::ID,
            new_admin,
            pool_state: POOL_STATE_ID,
        }
    }
}
