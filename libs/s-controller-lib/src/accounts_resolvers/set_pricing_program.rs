use s_controller_interface::{SControllerError, SetPricingProgramKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{find_pool_state_address, program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct SetPricingProgramFreeArgs<S> {
    pub new_pricing_program: Pubkey,
    pub pool_state_acc: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> SetPricingProgramFreeArgs<S> {
    pub fn resolve(&self) -> Result<SetPricingProgramKeys, SControllerError> {
        if *self.pool_state_acc.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        self.resolve_with_pool_state_id(POOL_STATE_ID)
    }
}

impl<S: ReadonlyAccountData> SetPricingProgramFreeArgs<S> {
    pub fn resolve_for_prog(
        &self,
        program_id: Pubkey,
    ) -> Result<SetPricingProgramKeys, SControllerError> {
        self.resolve_with_pool_state_id(find_pool_state_address(program_id).0)
    }

    pub fn resolve_with_pool_state_id(
        &self,
        pool_state_id: Pubkey,
    ) -> Result<SetPricingProgramKeys, SControllerError> {
        let pool_state_data = self.pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(SetPricingProgramKeys {
            admin: pool_state.admin,
            new_pricing_program: self.new_pricing_program,
            pool_state: pool_state_id,
        })
    }
}
