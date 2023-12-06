use s_controller_interface::{DisableLstInputKeys, LstState, PoolState, SControllerError};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_pool_state,
};

pub struct DisableLstInputFreeArgs<
    S: ReadonlyAccountData + KeyedAccount,
    L: ReadonlyAccountData + KeyedAccount,
> {
    pub lst_index: usize,
    pub pool_state: S,
    pub lst_state_list: L,
}

impl<S: ReadonlyAccountData + KeyedAccount, L: ReadonlyAccountData + KeyedAccount>
    DisableLstInputFreeArgs<S, L>
{
    pub fn resolve(&self) -> Result<DisableLstInputKeys, SControllerError> {
        let Self {
            lst_index,
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
        } = self;
        if *pool_state_account.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        if *lst_state_list_account.key() != LST_STATE_LIST_ID {
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

        Ok(DisableLstInputKeys {
            admin: *admin,
            lst_mint: *mint,
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
        })
    }
}

/// Iterates through lst_state_list to find lst_index.
/// Suitable for use on client-side.
/// Does not check identity of pool_state and lst_state_list
pub struct DisableLstInputByMintFreeArgs<S: ReadonlyAccountData, L: ReadonlyAccountData> {
    pub lst_mint: Pubkey,
    pub pool_state: S,
    pub lst_state_list: L,
}

impl<S: ReadonlyAccountData, L: ReadonlyAccountData> DisableLstInputByMintFreeArgs<S, L> {
    /// Returns (keys, index of lst_mint in lst_state_list)
    pub fn resolve(&self) -> Result<(DisableLstInputKeys, usize), SControllerError> {
        let Self {
            lst_mint,
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
        } = self;
        let lst_state_list_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_data)?;
        let (index, LstState { mint, .. }) = try_find_lst_mint_on_list(*lst_mint, lst_state_list)?;

        let pool_state_data = pool_state_account.data();
        let pool_state = try_pool_state(&pool_state_data)?;
        let PoolState { admin, .. } = pool_state;

        Ok((
            DisableLstInputKeys {
                admin: *admin,
                lst_mint: *mint,
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
            },
            index,
        ))
    }
}
