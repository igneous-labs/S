use std::{collections::HashMap, error::Error};

use jupiter_amm_interface::SwapParams;
use sanctum_lst_list::{PoolInfo, SanctumLst, SplPoolAccounts};
use solana_sdk::{account::Account, instruction::Instruction, pubkey::Pubkey, system_program};
use spl_token::native_mint;
use stakedex_interface::{stake_wrapped_sol_ix, StakeWrappedSolIxArgs, StakeWrappedSolKeys};
use stakedex_marinade::MarinadeStakedex;
use stakedex_sdk_common::{
    find_fee_token_acc, stakedex_program::SOL_BRIDGE_OUT_ID, wsol_bridge_in, DepositSol,
};
use stakedex_spl_stake_pool::SplStakePoolStakedex;

pub enum DepositSolStakedex {
    Marinade(MarinadeStakedex),
    SplLike(SplStakePoolStakedex),
}

impl DepositSolStakedex {
    pub fn from_sanctum_lst(
        SanctumLst {
            pool, name, symbol, ..
        }: &SanctumLst,
    ) -> Self {
        match pool {
            PoolInfo::Marinade => Self::Marinade(MarinadeStakedex::default()),
            PoolInfo::Spl(SplPoolAccounts {
                pool: stake_pool_addr,
                ..
            })
            | PoolInfo::SanctumSpl(SplPoolAccounts {
                pool: stake_pool_addr,
                ..
            })
            | PoolInfo::SanctumSplMulti(SplPoolAccounts {
                pool: stake_pool_addr,
                ..
            }) => {
                let stake_pool_program = pool.pool_program().into();
                Self::SplLike(SplStakePoolStakedex {
                    stake_pool_addr: *stake_pool_addr,
                    stake_pool_program,
                    stake_pool_label: name.clone(),
                    ..Default::default()
                })
            }
            _ => panic!("Deposit SOL unsupported for {symbol}"),
        }
    }

    /*
    // TODO: hacky, but this is not needed rn since
    // we dont need to fetch the full validator list for deposit sol
    // and spool.get_accounts_to_update_lsts_filtered should cover the pool
    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        let mut res = match self {
            Self::Marinade(p) => p.get_accounts_to_update(),
            Self::SplLike(p) => p.get_accounts_to_update(),
        };
        res.sort();
        res.dedup();
        res
    }
     */

    pub fn update(
        &mut self,
        account_map: &HashMap<Pubkey, Account>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::Marinade(p) => {
                let marinade_state = account_map
                    .get(&stakedex_sdk_common::marinade_state::ID)
                    .ok_or("marinade state missing")?;
                p.update_state(&marinade_state.data)?;
            }
            Self::SplLike(p) => {
                let pool = account_map.get(&p.stake_pool_addr).ok_or("pool missing")?;
                p.update_stake_pool(&pool.data)?;
            }
        };
        Ok(())
    }

    pub fn quote_deposit_sol(
        &self,
        lamports_to_deposit: u64,
    ) -> Result<u64, Box<dyn Error + Send + Sync + 'static>> {
        let quote = match self {
            Self::Marinade(p) => p.get_deposit_sol_quote(lamports_to_deposit),
            Self::SplLike(p) => p.get_deposit_sol_quote(lamports_to_deposit),
        }?;
        Ok(quote.out_amount)
    }

    pub fn deposit_sol_ix(
        &self,
        SwapParams {
            in_amount,
            destination_mint,
            source_token_account,
            destination_token_account,
            token_transfer_authority,
            ..
        }: &SwapParams,
    ) -> Result<Instruction, Box<dyn Error + Send + Sync + 'static>> {
        let mut ix = stake_wrapped_sol_ix(
            StakeWrappedSolKeys {
                user: *token_transfer_authority,
                wsol_from: *source_token_account,
                dest_token_to: *destination_token_account,
                wsol_bridge_in: wsol_bridge_in::ID,
                sol_bridge_out: SOL_BRIDGE_OUT_ID,
                dest_token_fee_token_account: find_fee_token_acc(destination_mint).0,
                dest_token_mint: *destination_mint,
                wsol_mint: native_mint::ID,
                token_program: spl_token::ID, // TODO: support token-22 LSTs
                system_program: system_program::ID,
            },
            StakeWrappedSolIxArgs { amount: *in_amount },
        )?;
        ix.accounts.extend(match self {
            Self::Marinade(p) => p.virtual_ix()?.accounts,
            Self::SplLike(p) => p.virtual_ix()?.accounts,
        });
        Ok(ix)
    }
}
