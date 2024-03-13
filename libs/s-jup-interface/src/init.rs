use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, try_lst_state_list, try_pool_state,
};
use sanctum_lst_list::SanctumLst;
use solana_program::pubkey::Pubkey;
use solana_readonly_account::ReadonlyAccountData;

use crate::{
    utils::{try_lst_data, try_pricing_prog},
    SPool,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SPoolInitKeys {
    pub lst_state_list: Pubkey,
    pub pool_state: Pubkey,
}

impl From<SPoolInitKeys> for [Pubkey; 2] {
    fn from(
        SPoolInitKeys {
            lst_state_list,
            pool_state,
        }: SPoolInitKeys,
    ) -> Self {
        [lst_state_list, pool_state]
    }
}

impl Default for SPoolInitKeys {
    fn default() -> Self {
        Self {
            lst_state_list: s_controller_lib::program::LST_STATE_LIST_ID,
            pool_state: s_controller_lib::program::POOL_STATE_ID,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SPoolInitAccounts<S, L> {
    pub lst_state_list: L,
    pub pool_state: S,
}

impl<S, L> SPool<S, L> {
    /// Gets the list of accounts that must be fetched first to initialize
    /// SPool by passing the result into [`Self::from_fetched_accounts`]
    pub fn init_keys(program_id: Pubkey) -> SPoolInitKeys {
        SPoolInitKeys {
            lst_state_list: find_lst_state_list_address(program_id).0,
            pool_state: find_pool_state_address(program_id).0,
        }
    }
}

impl<S, L: ReadonlyAccountData> SPool<S, L> {
    /// `Self`s created from this fn must be update_full() 2 more times before they can be used
    /// - first update fetches pool_state, updates various sol value calculator programs and pricing program
    /// - second update fetches LP token mint read from fetched pool_state
    pub fn from_lst_state_list_account(
        program_id: Pubkey,
        lst_state_list_account: L,
        lst_list: &[SanctumLst],
    ) -> anyhow::Result<Self> {
        let SPoolInitKeys {
            lst_state_list: lst_state_list_addr,
            pool_state: pool_state_addr,
        } = Self::init_keys(program_id);
        let lst_data_list = {
            let lst_state_list_account_data = lst_state_list_account.data();
            let lst_state_list = try_lst_state_list(&lst_state_list_account_data)?;
            lst_state_list
                .iter()
                .map(|lst_state| try_lst_data(lst_list, lst_state))
                .collect()
        };
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
}

impl<S: ReadonlyAccountData, L: ReadonlyAccountData> SPool<S, L> {
    /// `Self`s created from this fn must be update_full() 1 more time before they can be used.
    ///  - this update updates the various sol value calculator programs and pricing program
    pub fn from_init_accounts(
        program_id: Pubkey,
        SPoolInitAccounts {
            lst_state_list: lst_state_list_acc,
            pool_state: pool_state_acc,
        }: SPoolInitAccounts<S, L>,
        lst_list: &[SanctumLst],
    ) -> anyhow::Result<Self> {
        let pricing_prog = {
            let lst_state_list_acc_data = lst_state_list_acc.data();
            let lst_state_list = try_lst_state_list(&lst_state_list_acc_data)?;
            let pool_state_acc_data = pool_state_acc.data();
            let pool_state = try_pool_state(&pool_state_acc_data)?;
            try_pricing_prog(pool_state, lst_state_list)?
        };
        let mut res = Self::from_lst_state_list_account(program_id, lst_state_list_acc, lst_list)?;
        res.pool_state_account = Some(pool_state_acc);
        res.pricing_prog = Some(pricing_prog);
        Ok(res)
    }
}
