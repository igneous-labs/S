use s_controller_interface::{EnablePoolKeys, SControllerError};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct EnablePoolFreeArgs<S: ReadonlyAccountData + KeyedAccount> {
    pub pool_state_acc: S,
}

impl<S: ReadonlyAccountData + KeyedAccount> EnablePoolFreeArgs<S> {
    pub fn resolve(&self) -> Result<EnablePoolKeys, SControllerError> {
        if *self.pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = self.pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(EnablePoolKeys {
            admin: pool_state.admin,
            pool_state: *self.pool_state_acc.key(),
        })
    }
}
