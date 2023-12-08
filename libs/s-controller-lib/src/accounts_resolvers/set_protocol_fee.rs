use s_controller_interface::{SControllerError, SetProtocolFeeKeys};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct SetProtocolFeeFreeArgs<S: ReadonlyAccountData + KeyedAccount> {
    pub pool_state: S,
}

impl<S: ReadonlyAccountData + KeyedAccount> SetProtocolFeeFreeArgs<S> {
    pub fn resolve(self) -> Result<SetProtocolFeeKeys, SControllerError> {
        let SetProtocolFeeFreeArgs {
            pool_state: pool_state_acc,
        } = self;

        if *pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(SetProtocolFeeKeys {
            admin: pool_state.admin,
            pool_state: POOL_STATE_ID,
        })
    }
}
