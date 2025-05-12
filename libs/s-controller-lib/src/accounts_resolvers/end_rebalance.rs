use s_controller_interface::{
    EndRebalanceKeys, RebalanceRecord, SControllerError, StartRebalanceKeys,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    create_pool_reserves_address, index_to_usize,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID, REBALANCE_RECORD_ID},
    try_lst_state_list, try_match_lst_mint_on_list, try_pool_state, try_rebalance_record,
};

/// Requires an existing rebalance_record account.
/// Should only be used on-chain
#[derive(Clone, Copy, Debug)]
pub struct EndRebalanceFreeArgs<S, L, R, M> {
    pub pool_state: S,
    pub lst_state_list: L,
    pub rebalance_record: R,
    pub dst_lst_mint: M,
}

impl<
        S: ReadonlyAccountData + ReadonlyAccountPubkey,
        L: ReadonlyAccountData + ReadonlyAccountPubkey,
        R: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > EndRebalanceFreeArgs<S, L, R, M>
{
    /// Returns (keys, dst_lst_index)
    pub fn resolve(self) -> Result<(EndRebalanceKeys, usize), SControllerError> {
        if *self.pool_state.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        if *self.lst_state_list.pubkey() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }
        if *self.rebalance_record.pubkey() != REBALANCE_RECORD_ID {
            return Err(SControllerError::IncorrectRebalanceRecord);
        }

        let pool_state_data = self.pool_state.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let rebalance_record_acc_data = self.rebalance_record.data();
        let RebalanceRecord { dst_lst_index, .. } =
            try_rebalance_record(&rebalance_record_acc_data)?;
        let dst_lst_index = index_to_usize(*dst_lst_index)?;

        let dst_lst_state =
            try_match_lst_mint_on_list(*self.dst_lst_mint.pubkey(), list, dst_lst_index)?;
        let dst_pool_reserves =
            create_pool_reserves_address(dst_lst_state, *self.dst_lst_mint.owner())?;

        Ok((
            EndRebalanceKeys {
                rebalance_authority: pool_state.rebalance_authority,
                dst_lst_mint: dst_lst_state.mint,
                dst_pool_reserves,
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                rebalance_record: REBALANCE_RECORD_ID,
            },
            dst_lst_index,
        ))
    }
}

/// Creates a corresponding EndRebalanceKeys for a reference StartRebalanceKeys.
///
/// Suitable for use on client side.
///
/// Newtype to impl From<StartRebalanceKeys> for EndRebalanceKeys
#[derive(Clone, Copy, Debug)]
pub struct EndRebalanceFromStartRebalanceKeys<'a>(pub &'a StartRebalanceKeys);

impl EndRebalanceFromStartRebalanceKeys<'_> {
    pub fn resolve(self) -> EndRebalanceKeys {
        let Self(StartRebalanceKeys {
            rebalance_authority,
            dst_lst_mint,
            dst_pool_reserves,
            pool_state,
            lst_state_list,
            rebalance_record,
            ..
        }) = self;
        EndRebalanceKeys {
            rebalance_authority: *rebalance_authority,
            dst_lst_mint: *dst_lst_mint,
            dst_pool_reserves: *dst_pool_reserves,
            pool_state: *pool_state,
            lst_state_list: *lst_state_list,
            rebalance_record: *rebalance_record,
        }
    }
}
