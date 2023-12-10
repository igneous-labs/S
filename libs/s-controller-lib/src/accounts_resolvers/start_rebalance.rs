use s_controller_interface::{SControllerError, StartRebalanceKeys};
use solana_program::{pubkey::Pubkey, system_program, sysvar};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    create_pool_reserves_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID, REBALANCE_RECORD_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list, try_pool_state,
    SrcDstLstIndexes,
};

#[derive(Clone, Copy, Debug)]
pub struct StartRebalanceFreeArgs<
    SM: ReadonlyAccountOwner + KeyedAccount,
    DM: ReadonlyAccountOwner + KeyedAccount,
    S: ReadonlyAccountData + KeyedAccount,
    L: ReadonlyAccountData + KeyedAccount,
> {
    pub withdraw_to: Pubkey,
    pub src_lst_index: usize,
    pub dst_lst_index: usize,
    pub lst_state_list: L,
    pub pool_state: S,
    pub src_lst_mint: SM,
    pub dst_lst_mint: DM,
}

impl<
        SM: ReadonlyAccountOwner + KeyedAccount,
        DM: ReadonlyAccountOwner + KeyedAccount,
        S: ReadonlyAccountData + KeyedAccount,
        L: ReadonlyAccountData + KeyedAccount,
    > StartRebalanceFreeArgs<SM, DM, S, L>
{
    pub fn resolve(self) -> Result<StartRebalanceKeys, SControllerError> {
        if *self.lst_state_list.key() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }
        if *self.pool_state.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let src_lst_state =
            try_match_lst_mint_on_list(*self.src_lst_mint.key(), list, self.src_lst_index)?;
        let src_pool_reserves =
            create_pool_reserves_address(src_lst_state, *self.src_lst_mint.owner())?;

        let dst_lst_state =
            try_match_lst_mint_on_list(*self.dst_lst_mint.key(), list, self.dst_lst_index)?;
        let dst_pool_reserves =
            create_pool_reserves_address(dst_lst_state, *self.dst_lst_mint.owner())?;

        let pool_state_acc_data = self.pool_state.data();
        let pool_state = try_pool_state(&pool_state_acc_data)?;

        Ok(StartRebalanceKeys {
            rebalance_authority: pool_state.rebalance_authority,
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            rebalance_record: REBALANCE_RECORD_ID,
            src_lst_mint: src_lst_state.mint,
            dst_lst_mint: dst_lst_state.mint,
            src_pool_reserves,
            dst_pool_reserves,
            withdraw_to: self.withdraw_to,
            instructions: sysvar::instructions::ID,
            system_program: system_program::ID,
            src_lst_token_program: *self.src_lst_mint.owner(),
        })
    }
}

/// Iterates through lst_state_list to find the lst indexes.
/// Suitable for use on client side
#[derive(Clone, Copy, Debug)]
pub struct StartRebalanceByMintsFreeArgs<
    SM: ReadonlyAccountOwner + KeyedAccount,
    DM: ReadonlyAccountOwner + KeyedAccount,
    S: ReadonlyAccountData + KeyedAccount,
    L: ReadonlyAccountData + KeyedAccount,
> {
    pub withdraw_to: Pubkey,
    pub lst_state_list: L,
    pub pool_state: S,
    pub src_lst_mint: SM,
    pub dst_lst_mint: DM,
}

impl<
        SM: ReadonlyAccountOwner + KeyedAccount,
        DM: ReadonlyAccountOwner + KeyedAccount,
        S: ReadonlyAccountData + KeyedAccount,
        L: ReadonlyAccountData + KeyedAccount,
    > StartRebalanceByMintsFreeArgs<SM, DM, S, L>
{
    pub fn resolve(self) -> Result<(StartRebalanceKeys, SrcDstLstIndexes), SControllerError> {
        if *self.lst_state_list.key() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }
        if *self.pool_state.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let (src_lst_index, src_lst_state) =
            try_find_lst_mint_on_list(*self.src_lst_mint.key(), list)?;
        let src_pool_reserves =
            create_pool_reserves_address(src_lst_state, *self.src_lst_mint.owner())?;

        let (dst_lst_index, dst_lst_state) =
            try_find_lst_mint_on_list(*self.dst_lst_mint.key(), list)?;
        let dst_pool_reserves =
            create_pool_reserves_address(dst_lst_state, *self.dst_lst_mint.owner())?;

        let pool_state_acc_data = self.pool_state.data();
        let pool_state = try_pool_state(&pool_state_acc_data)?;

        Ok((
            StartRebalanceKeys {
                rebalance_authority: pool_state.rebalance_authority,
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                rebalance_record: REBALANCE_RECORD_ID,
                src_lst_mint: src_lst_state.mint,
                dst_lst_mint: dst_lst_state.mint,
                src_pool_reserves,
                dst_pool_reserves,
                withdraw_to: self.withdraw_to,
                instructions: sysvar::instructions::ID,
                system_program: system_program::ID,
                src_lst_token_program: *self.src_lst_mint.owner(),
            },
            SrcDstLstIndexes {
                src_lst_index,
                dst_lst_index,
            },
        ))
    }
}
