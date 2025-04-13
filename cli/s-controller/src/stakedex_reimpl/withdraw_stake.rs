use std::{collections::HashMap, error::Error, mem::size_of};

use jupiter_amm_interface::SwapParams;
use sanctum_lst_list::{PoolInfo, SanctumLst, SplPoolAccounts};
use solana_sdk::{
    account::Account,
    instruction::Instruction,
    pubkey::Pubkey,
    stake::{self, state::StakeStateV2},
    system_instruction::create_account_with_seed,
};
use stakedex_sdk_common::{
    BaseStakePoolAmm, WithdrawStakeIter, WithdrawStakeQuote, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};
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

    pub fn prefund_withdraw_stake_ixs(
        &self,
        swap_params: &SwapParams,
        quote: &WithdrawStakeQuote,
        info: &WithdrawStakeInfo,
    ) -> Result<Vec<Instruction>, Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::SplLike(sp) => spl_withdraw_stake_ix(sp, swap_params, quote, info),
        }
    }
}

#[derive(Debug)]
pub struct WithdrawStakeInfo {
    /// The stake account to withdraw to should be
    /// Pubkey::create_with_seed(base=token_transfer_authority, seed=seed, owner=stake_program)
    pub seed: String,
}

fn spl_withdraw_stake_ix(
    sp: &SplStakePoolStakedex,
    SwapParams {
        in_amount,
        source_token_account,
        token_transfer_authority,
        ..
    }: &SwapParams,
    WithdrawStakeQuote { voter, .. }: &WithdrawStakeQuote,
    WithdrawStakeInfo { seed }: &WithdrawStakeInfo,
) -> Result<Vec<Instruction>, Box<dyn Error + Send + Sync + 'static>> {
    let bridge_stake =
        Pubkey::create_with_seed(token_transfer_authority, seed, &stake::program::ID).unwrap();
    Ok(vec![
        create_account_with_seed(
            token_transfer_authority,
            &bridge_stake,
            token_transfer_authority,
            seed,
            STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
            size_of::<StakeStateV2>() as u64,
            &stake::program::ID,
        ),
        spl_stake_pool::instruction::withdraw_stake(
            &sp.program_id(),
            &sp.stake_pool_addr,
            &sp.stake_pool.validator_list,
            &sp.withdraw_authority_addr(),
            &spl_stake_pool::find_stake_program_address(
                &sp.program_id(),
                voter,
                &sp.stake_pool_addr,
                None,
            )
            .0,
            &bridge_stake,
            token_transfer_authority,
            token_transfer_authority,
            source_token_account,
            &sp.stake_pool.manager_fee_account,
            &sp.stake_pool.pool_mint,
            &sp.stake_pool.token_program_id,
            *in_amount,
        ),
    ])
}
