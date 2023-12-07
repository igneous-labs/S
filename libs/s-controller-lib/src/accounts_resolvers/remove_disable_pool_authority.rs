use s_controller_interface::{
    RemoveDisablePoolAuthorityIxArgs, RemoveDisablePoolAuthorityKeys, SControllerError,
};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{
    index_to_u32,
    program::{DISABLE_POOL_AUTHORITY_LIST_ID, POOL_STATE_ID},
    try_disable_pool_authority_list, try_find_element_in_list,
};

#[derive(Clone, Copy, Debug)]
pub struct RemoveDisablePoolAuthorityFreeArgs<
    S: ReadonlyAccountData + KeyedAccount,
    L: ReadonlyAccountData + KeyedAccount,
> {
    pub index: usize,
    pub refund_rent_to: Pubkey,
    pub signer: Pubkey,
    pub pool_state_acc: S,
    pub disable_pool_authority_list: L,
}

impl<S: ReadonlyAccountData + KeyedAccount, L: ReadonlyAccountData + KeyedAccount>
    RemoveDisablePoolAuthorityFreeArgs<S, L>
{
    pub fn resolve(&self) -> Result<RemoveDisablePoolAuthorityKeys, SControllerError> {
        if *self.pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        if *self.disable_pool_authority_list.key() != DISABLE_POOL_AUTHORITY_LIST_ID {
            return Err(SControllerError::IncorrectDisablePoolAuthorityList);
        }

        let disable_pool_authority_list_data = self.disable_pool_authority_list.data();
        let list = try_disable_pool_authority_list(&disable_pool_authority_list_data)?;
        let authority = list
            .get(self.index)
            .ok_or(SControllerError::InvalidDisablePoolAuthorityIndex)?;

        Ok(RemoveDisablePoolAuthorityKeys {
            refund_rent_to: self.refund_rent_to,
            signer: self.signer,
            pool_state: POOL_STATE_ID,
            authority: *authority,
            disable_pool_authority_list: DISABLE_POOL_AUTHORITY_LIST_ID,
        })
    }
}

/// Iterates through disable_pool_authority_list to find the index.
/// Does not check identity of pool_state_account and disable_pool_authority_list
/// Suitable for use client-side.
#[derive(Clone, Copy, Debug)]
pub struct RemoveDisablePoolAuthorityByPubkeyFreeArgs<
    S: ReadonlyAccountData + KeyedAccount,
    L: ReadonlyAccountData + KeyedAccount,
> {
    pub refund_rent_to: Pubkey,
    pub signer: Pubkey,
    pub authority: Pubkey,
    pub pool_state_acc: S,
    pub disable_pool_authority_list: L,
}

impl<S: ReadonlyAccountData + KeyedAccount, L: ReadonlyAccountData + KeyedAccount>
    RemoveDisablePoolAuthorityByPubkeyFreeArgs<S, L>
{
    pub fn resolve(
        &self,
    ) -> Result<
        (
            RemoveDisablePoolAuthorityKeys,
            RemoveDisablePoolAuthorityIxArgs,
        ),
        SControllerError,
    > {
        if *self.pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let disable_pool_authority_list_data = self.disable_pool_authority_list.data();
        let list = try_disable_pool_authority_list(&disable_pool_authority_list_data)?;
        let (index, _authority) = try_find_element_in_list(self.authority, list)
            .ok_or(SControllerError::InvalidDisablePoolAuthority)?;

        Ok((
            RemoveDisablePoolAuthorityKeys {
                refund_rent_to: self.refund_rent_to,
                signer: self.signer,
                authority: self.authority,
                pool_state: *self.pool_state_acc.key(),
                disable_pool_authority_list: DISABLE_POOL_AUTHORITY_LIST_ID,
            },
            RemoveDisablePoolAuthorityIxArgs {
                index: index_to_u32(index)?,
            },
        ))
    }
}
