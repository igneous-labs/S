use s_controller_interface::{SControllerError, SyncSolValueIxArgs, SyncSolValueKeys};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    create_pool_reserves_address,
    program::{LST_STATE_LIST_ID, STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list,
};

pub struct SyncSolValueFreeArgs<
    I: TryInto<usize>,
    L: ReadonlyAccountData + KeyedAccount,
    M: ReadonlyAccountOwner + KeyedAccount,
> {
    pub lst_index: I,
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<
        I: TryInto<usize>,
        L: ReadonlyAccountData + KeyedAccount,
        M: ReadonlyAccountOwner + KeyedAccount,
    > SyncSolValueFreeArgs<I, L, M>
{
    pub fn resolve(self) -> Result<SyncSolValueKeys, SControllerError> {
        if *self.lst_state_list.key() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }
        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let lst_state = try_match_lst_mint_on_list(*self.lst_mint.key(), list, self.lst_index)?;
        let pool_reserves = create_pool_reserves_address(lst_state, &self.lst_mint)?;

        Ok(SyncSolValueKeys {
            lst_mint: lst_state.mint,
            pool_state: STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            pool_reserves,
        })
    }
}

/// Iterates through lst_state_list to find lst_index.
/// Suitable for use on client-side
pub struct SyncSolValueByMintFreeArgs<
    L: ReadonlyAccountData,
    M: ReadonlyAccountOwner + KeyedAccount,
> {
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<L: ReadonlyAccountData, M: ReadonlyAccountOwner + KeyedAccount>
    SyncSolValueByMintFreeArgs<L, M>
{
    pub fn resolve(self) -> Result<(SyncSolValueKeys, SyncSolValueIxArgs), SControllerError> {
        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let (lst_index, lst_state) = try_find_lst_mint_on_list(*self.lst_mint.key(), list)?;
        let pool_reserves = create_pool_reserves_address(lst_state, &self.lst_mint)?;

        Ok((
            SyncSolValueKeys {
                lst_mint: *self.lst_mint.key(),
                pool_state: STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                pool_reserves,
            },
            SyncSolValueIxArgs {
                lst_index: lst_index
                    .try_into()
                    .map_err(|_e| SControllerError::MathError)?,
            },
        ))
    }
}
