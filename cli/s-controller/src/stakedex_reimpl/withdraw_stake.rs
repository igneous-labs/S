use std::{collections::HashMap, error::Error};

use sanctum_lst_list::{PoolInfo, SanctumLst, SplPoolAccounts};
use solana_sdk::{account::Account, pubkey::Pubkey};
use stakedex_sdk_common::{BaseStakePoolAmm, WithdrawStakeIter, WithdrawStakeQuote};
use stakedex_spl_stake_pool::{SplStakePoolStakedex, SplStakePoolStakedexInitKeys};

pub enum WithdrawStakeStakedex {
    SplLike(SplStakePoolStakedex),
    // TODO: Lido, but we're probably not using that for now
}

impl WithdrawStakeStakedex {
    pub fn from_sanctum_lst(SanctumLst { pool, symbol, .. }: &SanctumLst) -> Self {
        match pool {
            PoolInfo::Spl(SplPoolAccounts {
                pool: stake_pool_addr,
                validator_list,
                ..
            })
            | PoolInfo::SanctumSpl(SplPoolAccounts {
                pool: stake_pool_addr,
                validator_list,
                ..
            })
            | PoolInfo::SanctumSplMulti(SplPoolAccounts {
                pool: stake_pool_addr,
                validator_list,
                ..
            }) => {
                let stake_pool_program = pool.pool_program().into();
                let mut inner = SplStakePoolStakedex::new_uninitialized(
                    SplStakePoolStakedexInitKeys {
                        stake_pool_program,
                        stake_pool_addr: *stake_pool_addr,
                    },
                    Default::default(),
                );
                // so that we only need one more get_accounts_to_update() + update()
                inner.stake_pool.validator_list = *validator_list;
                Self::SplLike(inner)
            }
            _ => panic!("Withdraw stake unsupported for {symbol}"),
        }
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        match self {
            Self::SplLike(p) => p.get_accounts_to_update(),
        }
    }

    pub fn update(
        &mut self,
        account_map: &HashMap<Pubkey, Account>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::SplLike(p) => p.update(account_map),
        }?;
        Ok(())
    }

    /// Uses `get_withdraw_stake_quote_unchecked()` under the hood.
    /// This means this assumes the pool has already been updated for this epoch.
    pub fn withdraw_stake_quote_iter_dyn(
        &self,
        withdraw_amount: u64,
    ) -> Box<dyn Iterator<Item = WithdrawStakeQuote> + '_> {
        Box::new(match self {
            Self::SplLike(p) => p.withdraw_stake_quote_iter(withdraw_amount),
        })
    }
}
