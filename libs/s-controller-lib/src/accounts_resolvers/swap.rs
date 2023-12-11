use s_controller_interface::{SControllerError, SwapExactInKeys, SwapExactOutKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    create_pool_reserves_address, create_protocol_fee_accumulator_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID},
    try_find_lst_mint_on_list, try_lst_state_list, try_match_lst_mint_on_list, SrcDstLstIndexes,
};

pub struct SwapFreeArgs<
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    L: ReadonlyAccountData + ReadonlyAccountPubkey,
> {
    pub src_lst_index: usize,
    pub dst_lst_index: usize,
    pub signer: Pubkey,
    pub src_lst_acc: Pubkey,
    pub dst_lst_acc: Pubkey,
    pub src_lst_mint: SM,
    pub dst_lst_mint: DM,
    pub lst_state_list: L,
}

struct SwapComputedKeys {
    pub src_pool_reserves: Pubkey,
    pub dst_pool_reserves: Pubkey,
    pub protocol_fee_accumulator: Pubkey,
}

impl<
        SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
        DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
        L: ReadonlyAccountData + ReadonlyAccountPubkey,
    > SwapFreeArgs<SM, DM, L>
{
    fn compute_keys(&self) -> Result<SwapComputedKeys, SControllerError> {
        let Self {
            lst_state_list: lst_state_list_account,
            src_lst_mint,
            dst_lst_mint,
            src_lst_index,
            dst_lst_index,
            ..
        } = self;
        if *lst_state_list_account.pubkey() != LST_STATE_LIST_ID {
            return Err(SControllerError::IncorrectLstStateList);
        }

        let lst_state_list_acc_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;

        let src_lst_state =
            try_match_lst_mint_on_list(*src_lst_mint.pubkey(), lst_state_list, *src_lst_index)?;
        let src_pool_reserves = create_pool_reserves_address(src_lst_state, *src_lst_mint.owner())?;

        let dst_lst_state =
            try_match_lst_mint_on_list(*dst_lst_mint.pubkey(), lst_state_list, *dst_lst_index)?;
        let dst_pool_reserves = create_pool_reserves_address(dst_lst_state, *dst_lst_mint.owner())?;
        let protocol_fee_accumulator =
            create_protocol_fee_accumulator_address(dst_lst_state, *dst_lst_mint.owner())?;

        Ok(SwapComputedKeys {
            src_pool_reserves,
            dst_pool_reserves,
            protocol_fee_accumulator,
        })
    }

    pub fn resolve_exact_in(&self) -> Result<SwapExactInKeys, SControllerError> {
        let SwapComputedKeys {
            src_pool_reserves,
            dst_pool_reserves,
            protocol_fee_accumulator,
        } = self.compute_keys()?;
        let Self {
            signer,
            src_lst_acc,
            dst_lst_acc,
            src_lst_mint,
            dst_lst_mint,
            ..
        } = self;
        Ok(SwapExactInKeys {
            signer: *signer,
            src_lst_mint: *src_lst_mint.pubkey(),
            dst_lst_mint: *dst_lst_mint.pubkey(),
            src_lst_acc: *src_lst_acc,
            dst_lst_acc: *dst_lst_acc,
            protocol_fee_accumulator,
            src_lst_token_program: *src_lst_mint.owner(),
            dst_lst_token_program: *dst_lst_mint.owner(),
            pool_state: POOL_STATE_ID,
            lst_state_list: LST_STATE_LIST_ID,
            src_pool_reserves,
            dst_pool_reserves,
        })
    }

    pub fn resolve_exact_out(&self) -> Result<SwapExactOutKeys, SControllerError> {
        let SwapComputedKeys {
            src_pool_reserves,
            dst_pool_reserves,
            protocol_fee_accumulator,
        } = self.compute_keys()?;
        let Self {
            signer,
            src_lst_acc,
            dst_lst_acc,
            src_lst_mint,
            dst_lst_mint,
            ..
        } = self;
        Ok(SwapExactOutKeys {
            signer: *signer,
            src_lst_mint: *src_lst_mint.pubkey(),
            dst_lst_mint: *dst_lst_mint.pubkey(),
            src_lst_acc: *src_lst_acc,
            dst_lst_acc: *dst_lst_acc,
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
pub struct SwapByMintsFreeArgs<
    SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
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
        SM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
        DM: ReadonlyAccountOwner + ReadonlyAccountPubkey,
        L: ReadonlyAccountData,
    > SwapByMintsFreeArgs<SM, DM, L>
{
    fn compute_keys_and_indexes(
        &self,
    ) -> Result<(SwapComputedKeys, SrcDstLstIndexes), SControllerError> {
        let Self {
            lst_state_list: lst_state_list_account,
            src_lst_mint,
            dst_lst_mint,
            ..
        } = self;

        let lst_state_list_acc_data = lst_state_list_account.data();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;

        let (src_lst_index, src_lst_state) =
            try_find_lst_mint_on_list(*src_lst_mint.pubkey(), lst_state_list)?;
        let src_pool_reserves = create_pool_reserves_address(src_lst_state, *src_lst_mint.owner())?;

        let (dst_lst_index, dst_lst_state) =
            try_find_lst_mint_on_list(*dst_lst_mint.pubkey(), lst_state_list)?;
        let dst_pool_reserves = create_pool_reserves_address(dst_lst_state, *dst_lst_mint.owner())?;
        let protocol_fee_accumulator =
            create_protocol_fee_accumulator_address(dst_lst_state, *dst_lst_mint.owner())?;

        Ok((
            SwapComputedKeys {
                src_pool_reserves,
                dst_pool_reserves,
                protocol_fee_accumulator,
            },
            SrcDstLstIndexes {
                src_lst_index,
                dst_lst_index,
            },
        ))
    }

    pub fn resolve_exact_in(
        &self,
    ) -> Result<(SwapExactInKeys, SrcDstLstIndexes), SControllerError> {
        let (
            SwapComputedKeys {
                src_pool_reserves,
                dst_pool_reserves,
                protocol_fee_accumulator,
            },
            indexes,
        ) = self.compute_keys_and_indexes()?;

        let Self {
            signer,
            src_lst_acc,
            dst_lst_acc,
            src_lst_mint,
            dst_lst_mint,
            ..
        } = self;
        Ok((
            SwapExactInKeys {
                signer: *signer,
                src_lst_mint: *src_lst_mint.pubkey(),
                dst_lst_mint: *dst_lst_mint.pubkey(),
                src_lst_acc: *src_lst_acc,
                dst_lst_acc: *dst_lst_acc,
                protocol_fee_accumulator,
                src_lst_token_program: *src_lst_mint.owner(),
                dst_lst_token_program: *dst_lst_mint.owner(),
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                src_pool_reserves,
                dst_pool_reserves,
            },
            indexes,
        ))
    }

    pub fn resolve_exact_out(
        &self,
    ) -> Result<(SwapExactOutKeys, SrcDstLstIndexes), SControllerError> {
        let (
            SwapComputedKeys {
                src_pool_reserves,
                dst_pool_reserves,
                protocol_fee_accumulator,
            },
            indexes,
        ) = self.compute_keys_and_indexes()?;

        let Self {
            signer,
            src_lst_acc,
            dst_lst_acc,
            src_lst_mint,
            dst_lst_mint,
            ..
        } = self;
        Ok((
            SwapExactOutKeys {
                signer: *signer,
                src_lst_mint: *src_lst_mint.pubkey(),
                dst_lst_mint: *dst_lst_mint.pubkey(),
                src_lst_acc: *src_lst_acc,
                dst_lst_acc: *dst_lst_acc,
                protocol_fee_accumulator,
                src_lst_token_program: *src_lst_mint.owner(),
                dst_lst_token_program: *dst_lst_mint.owner(),
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                src_pool_reserves,
                dst_pool_reserves,
            },
            indexes,
        ))
    }
}
