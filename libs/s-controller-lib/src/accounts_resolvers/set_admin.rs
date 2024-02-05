use s_controller_interface::{SControllerError, SetAdminKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{find_pool_state_address, program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct SetAdminFreeArgs<S: ReadonlyAccountData + ReadonlyAccountPubkey> {
    pub new_admin: Pubkey,
    pub pool_state: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> SetAdminFreeArgs<S> {
    pub fn resolve(self) -> Result<SetAdminKeys, SControllerError> {
        self.resolve_inner(POOL_STATE_ID)
    }

    pub fn resolve_for_prog(self, program_id: Pubkey) -> Result<SetAdminKeys, SControllerError> {
        let pool_state_id = find_pool_state_address(program_id).0;
        self.resolve_inner(pool_state_id)
    }

    fn resolve_inner(self, pool_state_id: Pubkey) -> Result<SetAdminKeys, SControllerError> {
        let SetAdminFreeArgs {
            new_admin,
            pool_state: pool_state_acc,
        } = self;

        if *pool_state_acc.pubkey() != pool_state_id {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(SetAdminKeys {
            current_admin: pool_state.admin,
            new_admin,
            pool_state: pool_state_id,
        })
    }
}
