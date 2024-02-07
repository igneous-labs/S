use s_controller_interface::{SControllerError, SetSolValueCalculatorKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    create_pool_reserves_address, find_lst_state_list_address, find_pool_state_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list, try_pool_state,
};

#[derive(Clone, Copy, Debug)]
pub struct SetSolValueCalculatorFreeArgs<S, L, M> {
    pub lst_index: usize,
    pub pool_state: S,
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<
        S: ReadonlyAccountData + ReadonlyAccountPubkey,
        L: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > SetSolValueCalculatorFreeArgs<S, L, M>
{
    pub fn resolve(&self) -> Result<SetSolValueCalculatorKeys, SControllerError> {
        let Self {
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
            ..
        } = self;
        if *pool_state_account.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        if *lst_state_list_account.pubkey() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }
        self.resolve_with_pdas(POOL_STATE_ID, LST_STATE_LIST_ID)
    }
}

impl<
        S: ReadonlyAccountData,
        L: ReadonlyAccountData,
        M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > SetSolValueCalculatorFreeArgs<S, L, M>
{
    pub fn resolve_for_prog(
        &self,
        program_id: Pubkey,
    ) -> Result<SetSolValueCalculatorKeys, SControllerError> {
        let pool_state_id = find_pool_state_address(program_id).0;
        let lst_state_list_id = find_lst_state_list_address(program_id).0;

        self.resolve_with_pdas(pool_state_id, lst_state_list_id)
    }

    pub fn resolve_with_pdas(
        &self,
        pool_state_id: Pubkey,
        lst_state_list_id: Pubkey,
    ) -> Result<SetSolValueCalculatorKeys, SControllerError> {
        let Self {
            lst_index,
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
            lst_mint,
        } = self;
        let lst_state_list_data = lst_state_list_acc.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_data)?;
        let lst_state = try_match_lst_mint_on_list(*lst_mint.pubkey(), lst_state_list, *lst_index)?;
        let pool_reserves = create_pool_reserves_address(lst_state, *lst_mint.owner())?;

        let pool_state_data = pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok(SetSolValueCalculatorKeys {
            admin: pool_state.admin,
            lst_mint: lst_state.mint,
            pool_state: pool_state_id,
            pool_reserves,
            lst_state_list: lst_state_list_id,
        })
    }
}

/// Iterates through lst_state_list to find lst_index.
/// Suitable for use on client-side.
/// Does not check identity of pool_state and lst_state_list
#[derive(Clone, Copy, Debug)]
pub struct SetSolValueCalculatorByMintFreeArgs<S, L, M> {
    pub pool_state: S,
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<
        S: ReadonlyAccountData,
        L: ReadonlyAccountData,
        M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > SetSolValueCalculatorByMintFreeArgs<S, L, M>
{
    /// Returns (keys, lst_index)
    pub fn resolve(&self) -> Result<(SetSolValueCalculatorKeys, usize), SControllerError> {
        self.resolve_with_pdas(POOL_STATE_ID, LST_STATE_LIST_ID)
    }

    pub fn resolve_for_prog(
        &self,
        program_id: Pubkey,
    ) -> Result<(SetSolValueCalculatorKeys, usize), SControllerError> {
        let pool_state_id = find_pool_state_address(program_id).0;
        let lst_state_list_id = find_lst_state_list_address(program_id).0;

        self.resolve_with_pdas(pool_state_id, lst_state_list_id)
    }
    pub fn resolve_with_pdas(
        &self,
        pool_state_id: Pubkey,
        lst_state_list_id: Pubkey,
    ) -> Result<(SetSolValueCalculatorKeys, usize), SControllerError> {
        let Self {
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
            lst_mint,
        } = self;

        let lst_state_list_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_data)?;
        let (lst_index, lst_state) = try_find_lst_mint_on_list(*lst_mint.pubkey(), lst_state_list)?;
        let pool_reserves = create_pool_reserves_address(lst_state, *lst_mint.owner())?;

        let pool_state_data = pool_state_account.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok((
            SetSolValueCalculatorKeys {
                admin: pool_state.admin,
                lst_mint: lst_state.mint,
                pool_state: pool_state_id,
                pool_reserves,
                lst_state_list: lst_state_list_id,
            },
            lst_index,
        ))
    }
}
