use std::{collections::HashMap, error::Error};

use jupiter_amm_interface::SwapParams;
use sanctum_lst_list::{PoolInfo, SanctumLst, SplPoolAccounts};
use solana_sdk::{
    account::Account, borsh0_10::try_from_slice_unchecked, instruction::Instruction, pubkey::Pubkey,
};
use spl_stake_pool::{
    error::StakePoolError,
    find_withdraw_authority_program_address,
    instruction::{withdraw_sol, withdraw_sol_with_authority},
    state::StakePool,
};
use stakedex_sdk_common::STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS;

/// Bit of a misnomer since we aren't actually going through stakedex program
/// to avoid additional fees
pub enum WithdrawSolStakedex {
    SplLike {
        stake_pool_program: Pubkey,
        stake_pool_addr: Pubkey,
        stake_pool: StakePool,
        // total account lamports, including rent-exemption
        reserves_lamports: u64,
    },
}

impl WithdrawSolStakedex {
    pub fn from_sanctum_lst(SanctumLst { pool, symbol, .. }: &SanctumLst) -> Self {
        match pool {
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
                Self::SplLike {
                    stake_pool_program,
                    stake_pool_addr: *stake_pool_addr,
                    stake_pool: Default::default(),
                    reserves_lamports: 0,
                }
            }
            _ => panic!("Deposit SOL unsupported for {symbol}"),
        }
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        let mut res = match self {
            Self::SplLike {
                stake_pool_addr,
                stake_pool: StakePool { reserve_stake, .. },
                ..
            } => vec![*stake_pool_addr, *reserve_stake],
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
            Self::SplLike {
                stake_pool_addr,
                stake_pool,
                reserves_lamports,
                ..
            } => {
                let pool = account_map.get(stake_pool_addr).ok_or("pool missing")?;
                *stake_pool = try_from_slice_unchecked::<StakePool>(&pool.data)?;
                let reserve_acc = account_map
                    .get(&stake_pool.reserve_stake)
                    .ok_or("reserves missing")?;
                *reserves_lamports = reserve_acc.lamports;
            }
        };
        Ok(())
    }

    /// Assumes the pool has already been updated for this epoch.
    pub fn quote_withdraw_sol(
        &self,
        lst_to_withdraw: u64,
    ) -> Result<u64, Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::SplLike {
                stake_pool,
                reserves_lamports,
                ..
            } => {
                let reserve_avail = reserves_lamports
                    .checked_sub(STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS)
                    .ok_or("reserves not fetched")?;

                let pool_tokens_fee = stake_pool
                    .calc_pool_tokens_sol_withdrawal_fee(lst_to_withdraw)
                    .ok_or(StakePoolError::CalculationFailure)?;
                let pool_tokens_burnt = lst_to_withdraw
                    .checked_sub(pool_tokens_fee)
                    .ok_or(StakePoolError::CalculationFailure)?;
                let withdraw_lamports = stake_pool
                    .calc_lamports_withdraw_amount(pool_tokens_burnt)
                    .ok_or(StakePoolError::CalculationFailure)?;
                if withdraw_lamports > reserve_avail {
                    Err(format!("reserves only has {reserve_avail} lamports to withdraw").into())
                } else {
                    Ok(withdraw_lamports)
                }
            }
        }
    }

    pub fn withdraw_sol_ix(
        &self,
        SwapParams {
            in_amount,
            source_token_account,
            destination_token_account,
            token_transfer_authority,
            ..
        }: &SwapParams,
    ) -> Result<Instruction, Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::SplLike {
                stake_pool_program,
                stake_pool_addr,
                stake_pool,
                ..
            } => Ok(
                // wtf man why dont they just expose withdraw_sol_internal()
                match stake_pool.sol_deposit_authority.as_ref() {
                    None => withdraw_sol(
                        stake_pool_program,
                        stake_pool_addr,
                        &find_withdraw_authority_program_address(
                            stake_pool_program,
                            stake_pool_addr,
                        )
                        .0,
                        token_transfer_authority,
                        source_token_account,
                        &stake_pool.reserve_stake,
                        destination_token_account,
                        &stake_pool.manager_fee_account,
                        &stake_pool.pool_mint,
                        &stake_pool.token_program_id,
                        *in_amount,
                    ),
                    Some(sol_withdraw_auth) => withdraw_sol_with_authority(
                        stake_pool_program,
                        stake_pool_addr,
                        sol_withdraw_auth,
                        &find_withdraw_authority_program_address(
                            stake_pool_program,
                            stake_pool_addr,
                        )
                        .0,
                        token_transfer_authority,
                        source_token_account,
                        &stake_pool.reserve_stake,
                        destination_token_account,
                        &stake_pool.manager_fee_account,
                        &stake_pool.pool_mint,
                        &stake_pool.token_program_id,
                        *in_amount,
                    ),
                },
            ),
        }
    }
}
