use s_controller_interface::{SControllerError, SetPricingProgramKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct SetPricingProgramFreeArgs<S: ReadonlyAccountData + KeyedAccount> {
    pub new_pricing_program: Pubkey,
    pub pool_state_acc: S,
}

impl<S: ReadonlyAccountData + KeyedAccount> SetPricingProgramFreeArgs<S> {
    pub fn resolve(&self) -> Result<SetPricingProgramKeys, SControllerError> {
        if *self.pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = self.pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(SetPricingProgramKeys {
            admin: pool_state.admin,
            new_pricing_program: self.new_pricing_program,
            pool_state: *self.pool_state_acc.key(),
        })
    }
}
