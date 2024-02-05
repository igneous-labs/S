use s_controller_interface::{AddDisablePoolAuthorityKeys, SControllerError};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{
    find_disable_pool_authority_list_address, find_pool_state_address,
    program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID},
    try_pool_state,
};

#[derive(Clone, Copy, Debug)]
pub struct AddDisablePoolAuthorityFreeArgs<S: ReadonlyAccountData + ReadonlyAccountPubkey> {
    pub payer: Pubkey,
    pub new_authority: Pubkey,
    pub pool_state_acc: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> AddDisablePoolAuthorityFreeArgs<S> {
    pub fn resolve(&self) -> Result<AddDisablePoolAuthorityKeys, SControllerError> {
        if *self.pool_state_acc.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = self.pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(AddDisablePoolAuthorityKeys {
            payer: self.payer,
            admin: pool_state.admin,
            pool_state: POOL_STATE_ID,
            new_authority: self.new_authority,
            disable_pool_authority_list: DISABLE_POOL_AUTHORITY_LIST_ID,
            system_program: system_program::ID,
        })
    }

    pub fn resolve_for_prog(
        &self,
        program_id: Pubkey,
    ) -> Result<AddDisablePoolAuthorityKeys, SControllerError> {
        let pool_state_data = self.pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(AddDisablePoolAuthorityKeys {
            payer: self.payer,
            admin: pool_state.admin,
            pool_state: find_pool_state_address(program_id).0,
            new_authority: self.new_authority,
            disable_pool_authority_list: find_disable_pool_authority_list_address(program_id).0,
            system_program: system_program::ID,
        })
    }
}
