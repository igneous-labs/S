use s_controller_interface::{RemoveLstIxArgs, RemoveLstKeys, SControllerError};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    create_pool_reserves_address, create_protocol_fee_accumulator_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID, PROTOCOL_FEE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list, try_pool_state,
};

/// Must ensure protocol_fee_accumulator and pool_reserves token accounts
/// are empty before calling
#[derive(Clone, Copy, Debug)]
pub struct RemoveLstFreeArgs<
    S: ReadonlyAccountData + KeyedAccount,
    L: ReadonlyAccountData + KeyedAccount,
    M: ReadonlyAccountOwner + KeyedAccount,
> {
    pub lst_index: usize,
    pub refund_rent_to: Pubkey,
    pub pool_state: S,
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<
        S: ReadonlyAccountData + KeyedAccount,
        L: ReadonlyAccountData + KeyedAccount,
        M: ReadonlyAccountOwner + KeyedAccount,
    > RemoveLstFreeArgs<S, L, M>
{
    pub fn resolve(self) -> Result<RemoveLstKeys, SControllerError> {
        let Self {
            lst_index,
            refund_rent_to,
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
            lst_mint,
        } = self;
        if *pool_state_account.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }
        if *lst_state_list_account.key() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }
        let lst_state_list_acc_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;

        let lst_state = try_match_lst_mint_on_list(*lst_mint.key(), lst_state_list, lst_index)?;

        let pool_reserves = create_pool_reserves_address(lst_state, *lst_mint.owner())?;
        let protocol_fee_accumulator =
            create_protocol_fee_accumulator_address(lst_state, *lst_mint.owner())?;

        let pool_state_acc_data = pool_state_account.data();
        let pool_state = try_pool_state(&pool_state_acc_data)?;

        Ok(RemoveLstKeys {
            admin: pool_state.admin,
            refund_rent_to,
            lst_mint: *lst_mint.key(),
            pool_reserves,
            protocol_fee_accumulator,
            protocol_fee_accumulator_auth: PROTOCOL_FEE_ID,
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            lst_token_program: *lst_mint.owner(),
        })
    }
}

/// Iterates through lst_state_list to find lst_index.
/// Suitable for use on client-side.
/// Does not check identity of pool_state and lst_state_list
#[derive(Clone, Copy, Debug)]
pub struct RemoveLstByMintFreeArgs<
    S: ReadonlyAccountData,
    L: ReadonlyAccountData,
    M: ReadonlyAccountOwner + KeyedAccount,
> {
    pub refund_rent_to: Pubkey,
    pub pool_state: S,
    pub lst_state_list: L,
    pub lst_mint: M,
}

impl<S: ReadonlyAccountData, L: ReadonlyAccountData, M: ReadonlyAccountOwner + KeyedAccount>
    RemoveLstByMintFreeArgs<S, L, M>
{
    /// Does not check identity of pool_state and lst_state_list
    pub fn resolve(self) -> Result<(RemoveLstKeys, RemoveLstIxArgs), SControllerError> {
        let RemoveLstByMintFreeArgs {
            refund_rent_to,
            pool_state: pool_state_account,
            lst_state_list: lst_state_list_account,
            lst_mint,
        } = self;

        let lst_state_list_acc_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;

        let (lst_index, lst_state) = try_find_lst_mint_on_list(*lst_mint.key(), lst_state_list)?;
        let pool_reserves = create_pool_reserves_address(lst_state, *lst_mint.owner())?;
        let protocol_fee_accumulator =
            create_protocol_fee_accumulator_address(lst_state, *lst_mint.owner())?;

        let pool_state_acc_data = pool_state_account.data();
        let pool_state = try_pool_state(&pool_state_acc_data)?;

        Ok((
            RemoveLstKeys {
                admin: pool_state.admin,
                refund_rent_to,
                lst_mint: *lst_mint.key(),
                pool_reserves,
                protocol_fee_accumulator,
                protocol_fee_accumulator_auth: PROTOCOL_FEE_ID,
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                lst_token_program: *lst_mint.owner(),
            },
            RemoveLstIxArgs {
                lst_index: lst_index
                    .try_into()
                    .map_err(|_e| SControllerError::MathError)?,
            },
        ))
    }
}
