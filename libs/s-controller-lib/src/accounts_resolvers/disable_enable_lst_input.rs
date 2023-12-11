use s_controller_interface::{
    DisableLstInputKeys, EnableLstInputKeys, LstState, PoolState, SControllerError,
};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_pool_state,
};

struct DisableEnableLstInputComputedKeys {
    pub admin: Pubkey,
    pub lst_mint: Pubkey,
}

pub struct DisableEnableLstInputFreeArgs<
    S: ReadonlyAccountData + ReadonlyAccountPubkey,
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
> {
    pub lst_index: usize,
    pub pool_state: S,
    pub lst_state_list: L,
}

impl<
        S: ReadonlyAccountData + ReadonlyAccountPubkey,
        L: ReadonlyAccountData + ReadonlyAccountPubkey,
    > DisableEnableLstInputFreeArgs<S, L>
{
    fn compute_keys(&self) -> Result<DisableEnableLstInputComputedKeys, SControllerError> {
        let Self {
            lst_index,
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
        } = self;
        if *pool_state_account.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        if *lst_state_list_account.pubkey() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }

        let lst_state_list_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_data)?;
        let LstState { mint, .. } = lst_state_list
            .get(*lst_index)
            .ok_or(SControllerError::InvalidLstIndex)?;

        let pool_state_data = pool_state_account.data();
        let pool_state = try_pool_state(&pool_state_data)?;
        let PoolState { admin, .. } = pool_state;

        Ok(DisableEnableLstInputComputedKeys {
            admin: *admin,
            lst_mint: *mint,
        })
    }

    pub fn resolve_disable(&self) -> Result<DisableLstInputKeys, SControllerError> {
        let DisableEnableLstInputComputedKeys { admin, lst_mint } = self.compute_keys()?;
        Ok(DisableLstInputKeys {
            admin,
            lst_mint,
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
        })
    }

    pub fn resolve_enable(&self) -> Result<EnableLstInputKeys, SControllerError> {
        let DisableEnableLstInputComputedKeys { admin, lst_mint } = self.compute_keys()?;
        Ok(EnableLstInputKeys {
            admin,
            lst_mint,
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
        })
    }
}

/// Iterates through lst_state_list to find lst_index.
/// Suitable for use on client-side.
/// Does not check identity of pool_state and lst_state_list
pub struct DisableEnableLstInputByMintFreeArgs<S: ReadonlyAccountData, L: ReadonlyAccountData> {
    pub lst_mint: Pubkey,
    pub pool_state: S,
    pub lst_state_list: L,
}

impl<S: ReadonlyAccountData, L: ReadonlyAccountData> DisableEnableLstInputByMintFreeArgs<S, L> {
    fn compute_keys_and_index(
        &self,
    ) -> Result<(DisableEnableLstInputComputedKeys, usize), SControllerError> {
        let Self {
            lst_mint,
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
        } = self;
        let lst_state_list_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_data)?;
        let (lst_index, LstState { mint, .. }) =
            try_find_lst_mint_on_list(*lst_mint, lst_state_list)?;

        let pool_state_data = pool_state_account.data();
        let pool_state = try_pool_state(&pool_state_data)?;
        let PoolState { admin, .. } = pool_state;

        Ok((
            DisableEnableLstInputComputedKeys {
                admin: *admin,
                lst_mint: *mint,
            },
            lst_index,
        ))
    }

    /// Returns (keys, index of lst_mint in lst_state_list)
    pub fn resolve_disable(&self) -> Result<(DisableLstInputKeys, usize), SControllerError> {
        let (DisableEnableLstInputComputedKeys { admin, lst_mint }, lst_index) =
            self.compute_keys_and_index()?;

        Ok((
            DisableLstInputKeys {
                admin,
                lst_mint,
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
            },
            lst_index,
        ))
    }

    pub fn resolve_enable(&self) -> Result<(EnableLstInputKeys, usize), SControllerError> {
        let (DisableEnableLstInputComputedKeys { admin, lst_mint }, lst_index) =
            self.compute_keys_and_index()?;

        Ok((
            EnableLstInputKeys {
                admin,
                lst_mint,
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
            },
            lst_index,
        ))
    }
}
