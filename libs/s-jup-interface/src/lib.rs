use anyhow::anyhow;
use s_controller_interface::{LstState, PoolState};
use s_controller_lib::{try_lst_state_list, try_pool_state};
use s_pricing_prog_aggregate::KnownPricingProg;
use s_sol_val_calc_prog_aggregate::KnownLstSolValCalc;
use sanctum_associated_token_lib::{CreateAtaAddressArgs, FindAtaAddressArgs};
use solana_program::pubkey::{Pubkey, PubkeyError};
use solana_sdk::account::Account;

pub(crate) mod utils;

mod core;
mod init;
mod jup_interface;
mod update;

pub use core::*;
pub use init::*;
pub use jup_interface::*;
pub use update::*;

#[derive(Debug, Clone)]
pub struct LstData {
    pub sol_val_calc: KnownLstSolValCalc,
    pub reserves_balance: Option<u64>,
    pub token_program: Pubkey,
}

#[derive(Debug, Clone)]
pub struct SPoolJup {
    pub program_id: Pubkey,
    pub lst_state_list_addr: Pubkey,
    pub pool_state_addr: Pubkey,
    pub lp_mint_supply: Option<u64>,
    // keep as raw Account to use with solana-readonly-account traits
    pub pool_state_account: Option<Account>,
    pub lst_state_list_account: Account,
    pub pricing_prog: Option<KnownPricingProg>,
    // indices match that of lst_state_list.
    // None means we don't know how to handle the given lst
    // this could be due to incomplete data or unknown LST sol value calculator program
    pub lst_data_list: Vec<Option<LstData>>,
}

impl Default for SPoolJup {
    fn default() -> Self {
        Self {
            program_id: s_controller_lib::program::ID,
            lst_state_list_addr: s_controller_lib::program::LST_STATE_LIST_ID,
            pool_state_addr: s_controller_lib::program::POOL_STATE_ID,
            lp_mint_supply: None,
            pool_state_account: None,
            pricing_prog: None,
            lst_state_list_account: Account::default(),
            lst_data_list: Vec::new(),
        }
    }
}

// More impl blocks in other files
impl SPoolJup {
    pub fn pool_state(&self) -> anyhow::Result<&PoolState> {
        let pool_state = self
            .pool_state_account
            .as_ref()
            .ok_or_else(|| anyhow!("Pool state not fetched"))?;
        Ok(try_pool_state(&pool_state.data)?)
    }

    pub fn lst_state_list(&self) -> anyhow::Result<&[LstState]> {
        Ok(try_lst_state_list(&self.lst_state_list_account.data)?)
    }

    pub fn pricing_prog(&self) -> anyhow::Result<&KnownPricingProg> {
        self.pricing_prog
            .as_ref()
            .ok_or_else(|| anyhow!("pricing program not fetched"))
    }

    pub fn pool_reserves_account(
        &self,
        LstState {
            mint,
            pool_reserves_bump,
            ..
        }: &LstState,
        LstData { token_program, .. }: &LstData,
    ) -> Result<Pubkey, PubkeyError> {
        CreateAtaAddressArgs {
            find_ata_args: FindAtaAddressArgs {
                wallet: self.pool_state_addr,
                mint: *mint,
                token_program: *token_program,
            },
            bump: *pool_reserves_bump,
        }
        .create_ata_address()
    }

    fn find_ready_lst(&self, lst_mint: Pubkey) -> anyhow::Result<(&LstState, &LstData)> {
        let (lst_state, lst_data) = self
            .lst_state_list()?
            .iter()
            .zip(self.lst_data_list.iter())
            .find(|(state, _data)| state.mint == lst_mint)
            .ok_or_else(|| anyhow!("LST {lst_mint} not on list"))?;
        let lst_data = lst_data
            .as_ref()
            .ok_or_else(|| anyhow!("LST {lst_mint} not supported"))?;
        Ok((lst_state, lst_data))
    }
}
