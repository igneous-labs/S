use s_controller_interface::{SControllerError, SetProtocolFeeBeneficiaryKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{find_pool_state_address, program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct SetProtocolFeeBeneficiaryFreeArgs<S> {
    pub new_beneficiary: Pubkey,
    pub pool_state: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> SetProtocolFeeBeneficiaryFreeArgs<S> {
    pub fn resolve(self) -> Result<SetProtocolFeeBeneficiaryKeys, SControllerError> {
        if *self.pool_state.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        self.resolve_with_pool_state_id(POOL_STATE_ID)
    }
}

impl<S: ReadonlyAccountData> SetProtocolFeeBeneficiaryFreeArgs<S> {
    pub fn resolve_for_prog(
        self,
        program_id: Pubkey,
    ) -> Result<SetProtocolFeeBeneficiaryKeys, SControllerError> {
        let pool_state_id = find_pool_state_address(program_id).0;
        self.resolve_with_pool_state_id(pool_state_id)
    }

    pub fn resolve_with_pool_state_id(
        self,
        pool_state_id: Pubkey,
    ) -> Result<SetProtocolFeeBeneficiaryKeys, SControllerError> {
        let SetProtocolFeeBeneficiaryFreeArgs {
            new_beneficiary,
            pool_state,
        } = self;

        let pool_state_data = pool_state.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(SetProtocolFeeBeneficiaryKeys {
            current_beneficiary: pool_state.protocol_fee_beneficiary,
            new_beneficiary,
            pool_state: pool_state_id,
        })
    }
}
