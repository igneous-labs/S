use s_controller_interface::{AddDisablePoolAuthorityKeys, SControllerError};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{
    program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID},
    try_pool_state,
};

#[derive(Clone, Copy, Debug)]
pub struct AddDisablePoolAuthorityFreeArgs<S: ReadonlyAccountData + KeyedAccount> {
    pub payer: Pubkey,
    pub new_authority: Pubkey,
    pub pool_state_acc: S,
}

impl<S: ReadonlyAccountData + KeyedAccount> AddDisablePoolAuthorityFreeArgs<S> {
    pub fn resolve(&self) -> Result<AddDisablePoolAuthorityKeys, SControllerError> {
        if *self.pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = self.pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(AddDisablePoolAuthorityKeys {
            payer: self.payer,
            admin: pool_state.admin,
            pool_state: *self.pool_state_acc.key(),
            new_authority: self.new_authority,
            disable_pool_authority_list: DISABLE_POOL_AUTHORITY_LIST_ID,
            system_program: system_program::ID,
        })
    }
}
