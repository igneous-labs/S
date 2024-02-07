use s_controller_interface::{SControllerError, SetProtocolFeeKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{find_pool_state_address, program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct SetProtocolFeeFreeArgs<S> {
    pub pool_state: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> SetProtocolFeeFreeArgs<S> {
    pub fn resolve(self) -> Result<SetProtocolFeeKeys, SControllerError> {
        if *self.pool_state.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        self.resolve_with_pool_state_id(POOL_STATE_ID)
    }
}
impl<S: ReadonlyAccountData> SetProtocolFeeFreeArgs<S> {
    pub fn resolve_for_prog(
        self,
        program_id: Pubkey,
    ) -> Result<SetProtocolFeeKeys, SControllerError> {
        let pool_state_id = find_pool_state_address(program_id).0;
        self.resolve_with_pool_state_id(pool_state_id)
    }

    pub fn resolve_with_pool_state_id(
        self,
        pool_state_id: Pubkey,
    ) -> Result<SetProtocolFeeKeys, SControllerError> {
        let SetProtocolFeeFreeArgs { pool_state } = self;

        let pool_state_data = pool_state.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(SetProtocolFeeKeys {
            admin: pool_state.admin,
            pool_state: pool_state_id,
        })
    }
}
