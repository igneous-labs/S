use s_controller_interface::{RemoveLiquidityKeys, SControllerError};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    create_pool_reserves_address, create_protocol_fee_accumulator_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list, try_pool_state,
    AddRemoveLiquidityProgramIds,
};

#[derive(Clone, Copy, Debug)]
pub struct RemoveLiquidityFreeArgs<
    S: ReadonlyAccountData + ReadonlyAccountPubkey,
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
    M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
> {
    pub lst_index: usize,
    pub signer: Pubkey,
    pub src_lp_acc: Pubkey,
    pub dst_lst_acc: Pubkey,
    pub pool_state: S,
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<
        S: ReadonlyAccountData + ReadonlyAccountPubkey,
        L: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > RemoveLiquidityFreeArgs<S, L, M>
{
    pub fn resolve(self) -> Result<RemoveLiquidityKeys, SControllerError> {
        let Self {
            lst_index,
            signer,
            src_lp_acc,
            dst_lst_acc,
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
            lst_mint,
        } = self;
        if *pool_state_account.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        if *lst_state_list_account.pubkey() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }

        let lst_state_list_acc_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;
        let lst_state = try_match_lst_mint_on_list(*lst_mint.pubkey(), lst_state_list, lst_index)?;
        let pool_reserves = create_pool_reserves_address(lst_state, *lst_mint.owner())?;
        let protocol_fee_accumulator =
            create_protocol_fee_accumulator_address(lst_state, *lst_mint.owner())?;

        let pool_state_data = pool_state_account.data();
        let pool_state = try_pool_state(&pool_state_data)?;
        Ok(RemoveLiquidityKeys {
            signer,
            lst_mint: *lst_mint.pubkey(),
            dst_lst_acc,
            src_lp_acc,
            lp_token_mint: pool_state.lp_token_mint,
            protocol_fee_accumulator,
            lst_token_program: *lst_mint.owner(),
            lp_token_program: spl_token::ID,
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            pool_reserves,
        })
    }
}

/// Iterates through lst_state_list to find lst_index.
/// Suitable for use on client-side.
/// Does not check identity of pool_state and lst_state_list
#[derive(Clone, Copy, Debug)]
pub struct RemoveLiquidityByMintFreeArgs<
    S: ReadonlyAccountData,
    L: ReadonlyAccountData,
    M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
> {
    pub signer: Pubkey,
    pub src_lp_acc: Pubkey,
    pub dst_lst_acc: Pubkey,
    pub pool_state: S,
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<
        S: ReadonlyAccountData,
        L: ReadonlyAccountData,
        M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > RemoveLiquidityByMintFreeArgs<S, L, M>
{
    /// Does not check identity of pool_state and lst_state_list
    /// Returns:
    /// (partial instructions keys, index of lst on lst_state_list, additional program IDs)
    pub fn resolve(
        self,
    ) -> Result<(RemoveLiquidityKeys, usize, AddRemoveLiquidityProgramIds), SControllerError> {
        let Self {
            signer,
            src_lp_acc,
            dst_lst_acc,
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
            lst_mint,
        } = self;
        let lst_state_list_acc_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;
        let (lst_index, lst_state) = try_find_lst_mint_on_list(*lst_mint.pubkey(), lst_state_list)?;
        let pool_reserves = create_pool_reserves_address(lst_state, *lst_mint.owner())?;
        let protocol_fee_accumulator =
            create_protocol_fee_accumulator_address(lst_state, *lst_mint.owner())?;

        let pool_state_data = pool_state_account.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        Ok((
            RemoveLiquidityKeys {
                signer,
                lst_mint: *lst_mint.pubkey(),
                src_lp_acc,
                dst_lst_acc,
                lp_token_mint: pool_state.lp_token_mint,
                protocol_fee_accumulator,
                lst_token_program: *lst_mint.owner(),
                lp_token_program: spl_token::ID,
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                pool_reserves,
            },
            lst_index,
            AddRemoveLiquidityProgramIds {
                lst_calculator_program_id: lst_state.sol_value_calculator,
                pricing_program_id: pool_state.pricing_program,
            },
        ))
    }
}
