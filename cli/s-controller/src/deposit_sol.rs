use std::{collections::HashMap, error::Error};

use jupiter_amm_interface::SwapParams;
use sanctum_lst_list::{PoolInfo, SanctumLst, SplPoolAccounts};
use solana_sdk::{account::Account, instruction::Instruction, pubkey::Pubkey};
use stakedex_marinade::MarinadeStakedex;
use stakedex_sdk_common::{BaseStakePoolAmm, DepositSol};
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

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        let mut res = match self {
            Self::Marinade(p) => p.get_accounts_to_update(),
            Self::SplLike(p) => p.get_accounts_to_update(),
        };
        res.sort();
        res.dedup();
        res
    }

    pub fn update(
        &mut self,
        account_map: &HashMap<Pubkey, Account>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::Marinade(p) => p.update(account_map)?,
            Self::SplLike(p) => p.update(account_map)?,
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
        swap_params: &SwapParams,
    ) -> Result<Instruction, Box<dyn Error + Send + Sync + 'static>> {
        //m
        todo!()
    }
}
