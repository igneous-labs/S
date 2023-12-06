use s_controller_interface::{DisablePoolKeys, SControllerError};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID};

#[derive(Clone, Copy, Debug)]
pub struct DisablePoolFreeArgs<
    S: ReadonlyAccountData + KeyedAccount,
    L: ReadonlyAccountData + KeyedAccount,
> {
    pub authority: Pubkey,
    pub pool_state_acc: S,
    pub disable_pool_authority_list: L,
}

impl<S: ReadonlyAccountData + KeyedAccount, L: ReadonlyAccountData + KeyedAccount>
    DisablePoolFreeArgs<S, L>
{
    pub fn resolve(&self) -> Result<DisablePoolKeys, SControllerError> {
        if *self.pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        Ok(DisablePoolKeys {
            authority: self.authority,
            pool_state: *self.pool_state_acc.key(),
            disable_pool_authority_list: DISABLE_POOL_AUTHORITY_LIST_ID,
        })
    }
}
