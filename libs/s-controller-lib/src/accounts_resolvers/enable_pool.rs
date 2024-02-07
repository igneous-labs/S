use s_controller_interface::{EnablePoolKeys, SControllerError};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{find_pool_state_address, program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct EnablePoolFreeArgs<S> {
    pub pool_state_acc: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> EnablePoolFreeArgs<S> {
    pub fn resolve(&self) -> Result<EnablePoolKeys, SControllerError> {
        if *self.pool_state_acc.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        self.resolve_with_pool_state_id(POOL_STATE_ID)
    }
}

impl<S: ReadonlyAccountData> EnablePoolFreeArgs<S> {
    pub fn resolve_for_prog(&self, program_id: Pubkey) -> Result<EnablePoolKeys, SControllerError> {
        let pool_state_id = find_pool_state_address(program_id).0;
        self.resolve_with_pool_state_id(pool_state_id)
    }

    pub fn resolve_with_pool_state_id(
        &self,
        pool_state_id: Pubkey,
    ) -> Result<EnablePoolKeys, SControllerError> {
        let pool_state_data = self.pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;
        Ok(EnablePoolKeys {
            admin: pool_state.admin,
            pool_state: pool_state_id,
        })
    }
}
