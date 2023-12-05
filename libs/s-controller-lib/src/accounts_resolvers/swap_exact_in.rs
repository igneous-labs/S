use s_controller_interface::{SControllerError, SwapExactInKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    create_pool_reserves_address, create_protocol_fee_accumulator_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list, SrcDstLstIndexes,
};

pub struct SwapExactInFreeArgs<
    SI: TryInto<usize>,
    DI: TryInto<usize>,
    SM: ReadonlyAccountOwner + KeyedAccount,
    DM: ReadonlyAccountOwner + KeyedAccount,
    L: ReadonlyAccountData + KeyedAccount,
> {
    pub signer: Pubkey,
    pub src_lst_acc: Pubkey,
    pub dst_lst_acc: Pubkey,
    pub src_lst_index: SI,
    pub dst_lst_index: DI,
    pub src_lst_mint: SM,
    pub dst_lst_mint: DM,
    pub lst_state_list: L,
}

impl<
        SI: TryInto<usize>,
        DI: TryInto<usize>,
        SM: ReadonlyAccountOwner + KeyedAccount,
        DM: ReadonlyAccountOwner + KeyedAccount,
        L: ReadonlyAccountData + KeyedAccount,
    > SwapExactInFreeArgs<SI, DI, SM, DM, L>
{
    pub fn resolve(self) -> Result<SwapExactInKeys, SControllerError> {
        let Self {
            signer,
            src_lst_acc,
            dst_lst_acc,
            src_lst_index,
            dst_lst_index,
            src_lst_mint,
            dst_lst_mint,
            lst_state_list: lst_state_list_account,
        } = self;
        if *lst_state_list_account.key() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }

        let lst_state_list_acc_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;

        let src_lst_state =
            try_match_lst_mint_on_list(*src_lst_mint.key(), lst_state_list, src_lst_index)?;
        let src_pool_reserves = create_pool_reserves_address(src_lst_state, *src_lst_mint.owner())?;

        let dst_lst_state =
            try_match_lst_mint_on_list(*dst_lst_mint.key(), lst_state_list, dst_lst_index)?;
        let dst_pool_reserves = create_pool_reserves_address(dst_lst_state, *dst_lst_mint.owner())?;
        let protocol_fee_accumulator =
            create_protocol_fee_accumulator_address(dst_lst_state, *dst_lst_mint.owner())?;

        Ok(SwapExactInKeys {
            signer,
            src_lst_mint: *src_lst_mint.key(),
            dst_lst_mint: *dst_lst_mint.key(),
            src_lst_acc,
            dst_lst_acc,
            protocol_fee_accumulator,
            src_lst_token_program: *src_lst_mint.owner(),
            dst_lst_token_program: *dst_lst_mint.owner(),
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            src_pool_reserves,
            dst_pool_reserves,
        })
    }
}

/// Iterates through lst_state_list to find the lst indexes.
/// Suitable for use on client side.
/// Does not check identity of lst_state_list
pub struct SwapExactInByMintFreeArgs<
    SM: ReadonlyAccountOwner + KeyedAccount,
    DM: ReadonlyAccountOwner + KeyedAccount,
    L: ReadonlyAccountData,
> {
    pub signer: Pubkey,
    pub src_lst_acc: Pubkey,
    pub dst_lst_acc: Pubkey,
    pub src_lst_mint: SM,
    pub dst_lst_mint: DM,
    pub lst_state_list: L,
}

impl<
        SM: ReadonlyAccountOwner + KeyedAccount,
        DM: ReadonlyAccountOwner + KeyedAccount,
        L: ReadonlyAccountData,
    > SwapExactInByMintFreeArgs<SM, DM, L>
{
    pub fn resolve(self) -> Result<(SwapExactInKeys, SrcDstLstIndexes), SControllerError> {
        let Self {
            signer,
            src_lst_acc,
            dst_lst_acc,
            src_lst_mint,
            dst_lst_mint,
            lst_state_list: lst_state_list_account,
        } = self;

        let lst_state_list_acc_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;

        let (src_lst_index, src_lst_state) =
            try_find_lst_mint_on_list(*src_lst_mint.key(), lst_state_list)?;
        let src_pool_reserves = create_pool_reserves_address(src_lst_state, *src_lst_mint.owner())?;

        let (dst_lst_index, dst_lst_state) =
            try_find_lst_mint_on_list(*dst_lst_mint.key(), lst_state_list)?;
        let dst_pool_reserves = create_pool_reserves_address(dst_lst_state, *dst_lst_mint.owner())?;
        let protocol_fee_accumulator =
            create_protocol_fee_accumulator_address(dst_lst_state, *dst_lst_mint.owner())?;

        Ok((
            SwapExactInKeys {
                signer,
                src_lst_mint: *src_lst_mint.key(),
                dst_lst_mint: *dst_lst_mint.key(),
                src_lst_acc,
                dst_lst_acc,
                protocol_fee_accumulator,
                src_lst_token_program: *src_lst_mint.owner(),
                dst_lst_token_program: *dst_lst_mint.owner(),
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                src_pool_reserves,
                dst_pool_reserves,
            },
            SrcDstLstIndexes {
                src_lst_index,
                dst_lst_index,
            },
        ))
    }
}
