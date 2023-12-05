use s_controller_interface::{RemoveDisablePoolAuthorityKeys, SControllerError};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{
    program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID},
    try_pool_state,
};

#[derive(Clone, Copy, Debug)]
pub struct RemoveDisablePoolAuthorityFreeArgs<S: ReadonlyAccountData + KeyedAccount> {
    pub refund_rent_to: Pubkey,
    pub authority: Pubkey,
    pub pool_state_acc: S,
}

impl<S: ReadonlyAccountData + KeyedAccount> RemoveDisablePoolAuthorityFreeArgs<S> {
    pub fn resolve(&self) -> Result<RemoveDisablePoolAuthorityKeys, SControllerError> {
        if *self.pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = self.pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(RemoveDisablePoolAuthorityKeys {
            refund_rent_to: self.refund_rent_to,
            admin: pool_state.admin,
            pool_state: *self.pool_state_acc.key(),
            authority: self.authority,
            disable_pool_authority_list: DISABLE_POOL_AUTHORITY_LIST_ID,
        })
    }
}
