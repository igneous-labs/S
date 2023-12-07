use s_controller_interface::{SControllerError, SetProtocolFeeBeneficiaryKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct SetProtocolFeeBeneficiaryFreeArgs<S: ReadonlyAccountData + KeyedAccount> {
    pub new_beneficiary: Pubkey,
    pub pool_state: S,
}

impl<S: ReadonlyAccountData + KeyedAccount> SetProtocolFeeBeneficiaryFreeArgs<S> {
    pub fn resolve(self) -> Result<SetProtocolFeeBeneficiaryKeys, SControllerError> {
        let SetProtocolFeeBeneficiaryFreeArgs {
            new_beneficiary,
            pool_state: pool_state_acc,
        } = self;

        if *pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(SetProtocolFeeBeneficiaryKeys {
            current_beneficiary: pool_state.protocol_fee_beneficiary,
            new_beneficiary,
            pool_state: POOL_STATE_ID,
        })
    }
}
