use anyhow::anyhow;
use jupiter_amm_interface::AccountMap;
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, try_lst_state_list, try_pool_state,
};
use sanctum_lst_list::SanctumLst;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;

use crate::{
    utils::{try_lst_data, try_pricing_prog},
    SPoolJup,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SPoolInitAccounts {
    pub lst_state_list: Pubkey,
    pub pool_state: Pubkey,
}

impl From<SPoolInitAccounts> for [Pubkey; 2] {
    fn from(
        SPoolInitAccounts {
            lst_state_list,
            pool_state,
        }: SPoolInitAccounts,
    ) -> Self {
        [lst_state_list, pool_state]
    }
}

impl Default for SPoolInitAccounts {
    fn default() -> Self {
        Self {
            lst_state_list: s_controller_lib::program::LST_STATE_LIST_ID,
            pool_state: s_controller_lib::program::POOL_STATE_ID,
        }
    }
}

impl SPoolJup {
    /// Gets the list of accounts that must be fetched first to initialize
    /// SPool by passing the result into [`Self::from_fetched_accounts`]
    pub fn init_accounts(program_id: Pubkey) -> SPoolInitAccounts {
        SPoolInitAccounts {
            lst_state_list: find_lst_state_list_address(program_id).0,
            pool_state: find_pool_state_address(program_id).0,
        }
    }

    /// `Self`s created from this fn must be updated 2 more times before they can be used
    /// - first update fetches pool_state
    /// - second update fetches LP token mint read from fetched pool_state
    pub fn from_lst_state_list_account(
        program_id: Pubkey,
        lst_state_list_account: Account,
        lst_list: &[SanctumLst],
    ) -> anyhow::Result<Self> {
        let SPoolInitAccounts {
            lst_state_list: lst_state_list_addr,
            pool_state: pool_state_addr,
        } = Self::init_accounts(program_id);
        let lst_state_list = try_lst_state_list(&lst_state_list_account.data)?;
        let lst_data_list = lst_state_list
            .iter()
            .map(|lst_state| try_lst_data(lst_list, lst_state))
            .collect();
        Ok(Self {
            program_id,
            lst_state_list_addr,
            pool_state_addr,
            pool_state_account: None,
            pricing_prog: None,
            lp_mint_supply: None,
            lst_state_list_account,
            lst_data_list,
        })
    }

    /// `AccountMap` must contain accounts in [Self::init_accounts]
    pub fn from_fetched_accounts(
        program_id: Pubkey,
        accounts: &AccountMap,
        lst_list: &[SanctumLst],
    ) -> anyhow::Result<Self> {
        let SPoolInitAccounts {
            lst_state_list: lst_state_list_addr,
            pool_state: pool_state_addr,
        } = Self::init_accounts(program_id);

        let lst_state_list_acc = accounts
            .get(&lst_state_list_addr)
            .ok_or_else(|| anyhow!("Missing LST state list {lst_state_list_addr}"))?;
        let lst_state_list = Vec::from(try_lst_state_list(&lst_state_list_acc.data)?);
        let pool_state_acc = accounts
            .get(&pool_state_addr)
            .ok_or_else(|| anyhow!("Missing pool state {pool_state_addr}"))?;
        let pool_state = try_pool_state(&pool_state_acc.data)?;
        let pricing_prog = try_pricing_prog(pool_state, &lst_state_list)?;

        let mut res =
            Self::from_lst_state_list_account(program_id, lst_state_list_acc.clone(), lst_list)?;
        res.pool_state_account = Some(pool_state_acc.clone());
        res.pricing_prog = Some(pricing_prog);
        Ok(res)
    }
}
