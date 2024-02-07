use s_controller_interface::{SControllerError, SetRebalanceAuthorityKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{find_pool_state_address, program::POOL_STATE_ID, try_pool_state};

#[derive(Clone, Copy, Debug)]
pub struct SetRebalanceAuthorityFreeArgs {
    pub signer: Pubkey,
    pub new_rebalance_authority: Pubkey,
}

impl SetRebalanceAuthorityFreeArgs {
    pub fn resolve(self) -> SetRebalanceAuthorityKeys {
        self.resolve_with_pool_state_id(POOL_STATE_ID)
    }

    pub fn resolve_for_prog(self, program_id: Pubkey) -> SetRebalanceAuthorityKeys {
        self.resolve_with_pool_state_id(find_pool_state_address(program_id).0)
    }

    pub fn resolve_with_pool_state_id(self, pool_state_id: Pubkey) -> SetRebalanceAuthorityKeys {
        let Self {
            signer,
            new_rebalance_authority,
        } = self;
        SetRebalanceAuthorityKeys {
            signer,
            new_rebalance_authority,
            pool_state: pool_state_id,
        }
    }
}

/// For use client-side, does not check identity of pool_state
#[derive(Clone, Copy, Debug)]
pub struct KnownAuthoritySetRebalanceAuthorityFreeArgs<D> {
    pub new_rebalance_authority: Pubkey,
    pub pool_state: D,
}

impl<D: ReadonlyAccountData + ReadonlyAccountPubkey>
    KnownAuthoritySetRebalanceAuthorityFreeArgs<D>
{
    pub fn resolve_pool_admin_with_pool_state_id(
        &self,
    ) -> Result<SetRebalanceAuthorityKeys, SControllerError> {
        Ok(SetRebalanceAuthorityFreeArgs {
            signer: self.pool_admin()?,
            new_rebalance_authority: self.new_rebalance_authority,
        }
        .resolve_with_pool_state_id(*self.pool_state.pubkey()))
    }

    pub fn resolve_current_rebalance_authority_with_pool_state_id(
        &self,
    ) -> Result<SetRebalanceAuthorityKeys, SControllerError> {
        Ok(SetRebalanceAuthorityFreeArgs {
            signer: self.current_rebalance_authority()?,
            new_rebalance_authority: self.new_rebalance_authority,
        }
        .resolve_with_pool_state_id(*self.pool_state.pubkey()))
    }
}

impl<D: ReadonlyAccountData> KnownAuthoritySetRebalanceAuthorityFreeArgs<D> {
    pub fn resolve_pool_admin(&self) -> Result<SetRebalanceAuthorityKeys, SControllerError> {
        Ok(SetRebalanceAuthorityFreeArgs {
            signer: self.pool_admin()?,
            new_rebalance_authority: self.new_rebalance_authority,
        }
        .resolve())
    }

    pub fn resolve_pool_admin_for_prog(
        &self,
        program_id: Pubkey,
    ) -> Result<SetRebalanceAuthorityKeys, SControllerError> {
        Ok(SetRebalanceAuthorityFreeArgs {
            signer: self.pool_admin()?,
            new_rebalance_authority: self.new_rebalance_authority,
        }
        .resolve_for_prog(program_id))
    }

    pub fn resolve_current_rebalance_authority(
        &self,
    ) -> Result<SetRebalanceAuthorityKeys, SControllerError> {
        Ok(SetRebalanceAuthorityFreeArgs {
            signer: self.current_rebalance_authority()?,
            new_rebalance_authority: self.new_rebalance_authority,
        }
        .resolve())
    }

    pub fn resolve_current_rebalance_authority_for_prog(
        &self,
        program_id: Pubkey,
    ) -> Result<SetRebalanceAuthorityKeys, SControllerError> {
        Ok(SetRebalanceAuthorityFreeArgs {
            signer: self.current_rebalance_authority()?,
            new_rebalance_authority: self.new_rebalance_authority,
        }
        .resolve_for_prog(program_id))
    }

    fn pool_admin(&self) -> Result<Pubkey, SControllerError> {
        let pool_state_acc_data = self.pool_state.data();
        let pool_state = try_pool_state(&pool_state_acc_data)?;
        Ok(pool_state.admin)
    }

    fn current_rebalance_authority(&self) -> Result<Pubkey, SControllerError> {
        let pool_state_acc_data = self.pool_state.data();
        let pool_state = try_pool_state(&pool_state_acc_data)?;
        Ok(pool_state.rebalance_authority)
    }
}
