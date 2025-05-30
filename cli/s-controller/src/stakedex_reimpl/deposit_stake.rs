use std::{collections::HashMap, error::Error};

use jupiter_amm_interface::SwapParams;
use sanctum_lst_list::{
    marinade_program::MSOL_MINT_AUTH_ID, PoolInfo, SanctumLst, SplPoolAccounts,
};
use solana_sdk::{
    account::Account, instruction::Instruction, pubkey::Pubkey, stake, system_program, sysvar,
};
use stakedex_marinade::{validator_system::ValidatorRecordWrapper, MarinadeStakedex};
use stakedex_sdk_common::{
    marinade_state, msol, BaseStakePoolAmm, DepositStake, DepositStakeInfo, DepositStakeQuote,
    WithdrawStakeQuote,
};
use stakedex_spl_stake_pool::{SplStakePoolStakedex, SplStakePoolStakedexInitKeys};

pub enum DepositStakeStakedex {
    Marinade(MarinadeStakedex),
    SplLike(SplStakePoolStakedex),
    // TODO: UnstakeItStakedex, but we're probably not using that for now
}

impl DepositStakeStakedex {
    pub fn from_sanctum_lst(SanctumLst { pool, symbol, .. }: &SanctumLst) -> Self {
        match pool {
            PoolInfo::Marinade => Self::Marinade(MarinadeStakedex::default()),
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
            _ => panic!("Deposit stake unsupported for {symbol}"),
        }
    }

    pub fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        match self {
            Self::Marinade(p) => p.get_accounts_to_update(),
            Self::SplLike(p) => p.get_accounts_to_update(),
        }
    }

    pub fn update(
        &mut self,
        account_map: &HashMap<Pubkey, Account>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::Marinade(p) => p.update(account_map),
            Self::SplLike(p) => p.update(account_map),
        }?;
        Ok(())
    }

    /// Uses `get_deposit_stake_quote_unchecked()` under the hood to allow for
    /// quoting of pools with deposit authorities. This means this assumes the
    /// pool has already been updated for this epoch.
    ///
    /// Like the `stakedex-sdk` fn, returns Default::default() if no route found
    pub fn quote_deposit_stake(
        &self,
        withdraw_stake_quote: WithdrawStakeQuote,
    ) -> DepositStakeQuote {
        match self {
            Self::Marinade(p) => p.get_deposit_stake_quote_unchecked(withdraw_stake_quote),
            Self::SplLike(p) => p.get_deposit_stake_quote_unchecked(withdraw_stake_quote),
        }
    }

    pub fn deposit_stake_ixs(
        &self,
        swap_params: &SwapParams,
        quote: &DepositStakeQuote,
        deposit_stake_info: &DepositStakeInfo,
    ) -> Result<Vec<Instruction>, Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::Marinade(sp) => {
                marinade_deposit_stake_ixs(sp, swap_params, quote, deposit_stake_info)
            }
            Self::SplLike(sp) => spl_deposit_stake_ixs(sp, swap_params, quote, deposit_stake_info),
        }
    }
}

fn marinade_deposit_stake_ixs(
    m: &MarinadeStakedex,
    SwapParams {
        token_transfer_authority,
        destination_token_account,
        ..
    }: &SwapParams,
    quote: &DepositStakeQuote,
    deposit_stake_info: &DepositStakeInfo,
) -> Result<Vec<Instruction>, Box<dyn Error + Send + Sync + 'static>> {
    let validator_index = m
        .validator_records
        .iter()
        .position(|v| quote.voter == v.validator_account)
        .ok_or_else(|| format!("validator {} not in marinade", quote.voter))?;
    Ok(vec![marinade_finance_interface::deposit_stake_account_ix(
        marinade_finance_interface::DepositStakeAccountKeys {
            stake_account: deposit_stake_info.addr,
            stake_authority: *token_transfer_authority,
            state: m.main_state_key(),
            validator_list: m.state.validator_system.validator_list.account,
            stake_list: m.state.stake_system.stake_list.account,
            duplication_flag: ValidatorRecordWrapper::find_duplication_flag(
                &marinade_state::ID,
                &quote.voter,
            )
            .0,
            rent_payer: *token_transfer_authority,
            msol_mint: msol::ID,
            mint_to: *destination_token_account,
            msol_mint_authority: MSOL_MINT_AUTH_ID,
            clock: sysvar::clock::ID,
            rent: sysvar::rent::ID,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            stake_program: stake::program::ID,
        },
        marinade_finance_interface::DepositStakeAccountIxArgs {
            validator_index: validator_index.try_into()?,
        },
    )?])
}

fn spl_deposit_stake_ixs(
    spl: &SplStakePoolStakedex,
    SwapParams {
        token_transfer_authority,
        destination_token_account,
        ..
    }: &SwapParams,
    quote: &DepositStakeQuote,
    deposit_stake_info: &DepositStakeInfo,
) -> Result<Vec<Instruction>, Box<dyn Error + Send + Sync + 'static>> {
    Ok(
        if spl.stake_pool.stake_deposit_authority == spl.deposit_authority_program_address {
            spl_stake_pool::instruction::deposit_stake(
                &spl.program_id(),
                &spl.stake_pool_addr,
                &spl.stake_pool.validator_list,
                &spl.withdraw_authority_addr(),
                &deposit_stake_info.addr,
                token_transfer_authority,
                &spl_stake_pool::find_stake_program_address(
                    &spl.program_id(),
                    &quote.voter,
                    &spl.stake_pool_addr,
                    None,
                )
                .0,
                &spl.stake_pool.reserve_stake,
                destination_token_account,
                &spl.stake_pool.manager_fee_account,
                destination_token_account,
                &spl.stake_pool.pool_mint,
                &spl.stake_pool.token_program_id,
            )
        } else {
            // permissioned pool
            spl_stake_pool::instruction::deposit_stake_with_authority(
                &spl.program_id(),
                &spl.stake_pool_addr,
                &spl.stake_pool.validator_list,
                &spl.stake_pool.stake_deposit_authority,
                &spl.withdraw_authority_addr(),
                &deposit_stake_info.addr,
                token_transfer_authority,
                &spl_stake_pool::find_stake_program_address(
                    &spl.program_id(),
                    &quote.voter,
                    &spl.stake_pool_addr,
                    None,
                )
                .0,
                &spl.stake_pool.reserve_stake,
                destination_token_account,
                &spl.stake_pool.manager_fee_account,
                destination_token_account,
                &spl.stake_pool.pool_mint,
                &spl.stake_pool.token_program_id,
            )
        },
    )
}
