use s_controller_interface::{SControllerError, SyncSolValueKeys};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    create_pool_reserves_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list,
};

#[derive(Clone, Copy, Debug)]
pub struct SyncSolValueFreeArgs<
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
    M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
> {
    pub lst_index: usize,
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<
        L: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > SyncSolValueFreeArgs<L, M>
{
    pub fn resolve(self) -> Result<SyncSolValueKeys, SControllerError> {
        if *self.lst_state_list.pubkey() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }
        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let lst_state = try_match_lst_mint_on_list(*self.lst_mint.pubkey(), list, self.lst_index)?;
        let pool_reserves = create_pool_reserves_address(lst_state, *self.lst_mint.owner())?;

        Ok(SyncSolValueKeys {
            lst_mint: lst_state.mint,
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            pool_reserves,
        })
    }
}

/// Iterates through lst_state_list to find lst_index.
/// Suitable for use on client-side
#[derive(Clone, Copy, Debug)]
pub struct SyncSolValueByMintFreeArgs<
    L: ReadonlyAccountData,
    M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
> {
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<L: ReadonlyAccountData, M: ReadonlyAccountOwner + ReadonlyAccountPubkey>
    SyncSolValueByMintFreeArgs<L, M>
{
    /// Does not check identity of pool_state and lst_state_list
    /// Returns (keys, index of lst_mint on lst_state_list)
    pub fn resolve(self) -> Result<(SyncSolValueKeys, usize), SControllerError> {
        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let (lst_index, lst_state) = try_find_lst_mint_on_list(*self.lst_mint.pubkey(), list)?;
        let pool_reserves = create_pool_reserves_address(lst_state, *self.lst_mint.owner())?;

        Ok((
            SyncSolValueKeys {
                lst_mint: *self.lst_mint.pubkey(),
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                pool_reserves,
            },
            lst_index,
        ))
    }
}
