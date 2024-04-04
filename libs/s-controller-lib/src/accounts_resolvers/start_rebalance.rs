use s_controller_interface::{PoolState, SControllerError, StartRebalanceKeys};
use solana_program::{pubkey::Pubkey, system_program, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    create_pool_reserves_address, find_lst_state_list_address, find_pool_state_address,
    find_rebalance_record_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID, REBALANCE_RECORD_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list, try_pool_state,
    SrcDstLstIndexes, SrcDstLstSolValueCalcProgramIds,
};

#[derive(Clone, Copy, Debug)]
pub struct RebalancePdas {
    pub pool_state: Pubkey,
    pub lst_state_list: Pubkey,
    pub rebalance_record: Pubkey,
}

impl RebalancePdas {
    pub fn find_for_program_id(program_id: Pubkey) -> Self {
        let (pool_state, _) = find_pool_state_address(program_id);
        let (lst_state_list, _) = find_lst_state_list_address(program_id);
        let (rebalance_record, _) = find_rebalance_record_address(program_id);
        Self {
            pool_state,
            lst_state_list,
            rebalance_record,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StartRebalanceFreeArgs<
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    S: ReadonlyAccountData + ReadonlyAccountPubkey,
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
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
        SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
        DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
        S: ReadonlyAccountData + ReadonlyAccountPubkey,
        L: ReadonlyAccountData + ReadonlyAccountPubkey,
    > StartRebalanceFreeArgs<SM, DM, S, L>
{
    pub fn resolve(self) -> Result<StartRebalanceKeys, SControllerError> {
        if *self.lst_state_list.pubkey() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }
        if *self.pool_state.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let src_lst_state =
            try_match_lst_mint_on_list(*self.src_lst_mint.pubkey(), list, self.src_lst_index)?;
        let src_pool_reserves =
            create_pool_reserves_address(src_lst_state, *self.src_lst_mint.owner())?;

        let dst_lst_state =
            try_match_lst_mint_on_list(*self.dst_lst_mint.pubkey(), list, self.dst_lst_index)?;
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
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    S: ReadonlyAccountData + ReadonlyAccountPubkey,
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
> {
    pub withdraw_to: Pubkey,
    pub lst_state_list: L,
    pub pool_state: S,
    pub src_lst_mint: SM,
    pub dst_lst_mint: DM,
}

impl<
        SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
        DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
        S: ReadonlyAccountData + ReadonlyAccountPubkey,
        L: ReadonlyAccountData + ReadonlyAccountPubkey,
    > StartRebalanceByMintsFreeArgs<SM, DM, S, L>
{
    pub fn resolve(
        self,
    ) -> Result<
        (
            StartRebalanceKeys,
            SrcDstLstIndexes,
            SrcDstLstSolValueCalcProgramIds,
        ),
        SControllerError,
    > {
        self.resolve_with_pdas(RebalancePdas {
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            rebalance_record: REBALANCE_RECORD_ID,
        })
    }

    pub fn resolve_for_prog(
        self,
        program_id: Pubkey,
    ) -> Result<
        (
            StartRebalanceKeys,
            SrcDstLstIndexes,
            SrcDstLstSolValueCalcProgramIds,
        ),
        SControllerError,
    > {
        self.resolve_with_pdas(RebalancePdas::find_for_program_id(program_id))
    }

    pub fn resolve_with_pdas(
        self,
        RebalancePdas {
            pool_state,
            lst_state_list,
            rebalance_record,
        }: RebalancePdas,
    ) -> Result<
        (
            StartRebalanceKeys,
            SrcDstLstIndexes,
            SrcDstLstSolValueCalcProgramIds,
        ),
        SControllerError,
    > {
        let lst_state_list_acc_data = self.lst_state_list.data();
        let list = try_lst_state_list(&lst_state_list_acc_data)?;

        let (src_lst_index, src_lst_state) =
            try_find_lst_mint_on_list(*self.src_lst_mint.pubkey(), list)?;
        let src_pool_reserves =
            create_pool_reserves_address(src_lst_state, *self.src_lst_mint.owner())?;

        let (dst_lst_index, dst_lst_state) =
            try_find_lst_mint_on_list(*self.dst_lst_mint.pubkey(), list)?;
        let dst_pool_reserves =
            create_pool_reserves_address(dst_lst_state, *self.dst_lst_mint.owner())?;

        let pool_state_acc_data = self.pool_state.data();
        let PoolState {
            rebalance_authority,
            ..
        } = try_pool_state(&pool_state_acc_data)?;

        Ok((
            StartRebalanceKeys {
                rebalance_authority: *rebalance_authority,
                pool_state,
                lst_state_list,
                rebalance_record,
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
            SrcDstLstSolValueCalcProgramIds {
                src_lst_calculator_program_id: src_lst_state.sol_value_calculator,
                dst_lst_calculator_program_id: dst_lst_state.sol_value_calculator,
            },
        ))
    }
}
