use generic_pool_calculator_lib::{U64FeeFloor, U64RatioFloor};
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{clock::Clock, program_error::ProgramError};
use spl_calculator_interface::{Fee, SplCalculatorError, SplStakePool};

#[derive(Debug, Clone)]
pub struct SplStakePoolCalc(pub SplStakePool);

impl SplStakePoolCalc {
    pub const fn verify_pool_updated_for_this_epoch(
        &self,
        clock: &Clock,
    ) -> Result<(), SplCalculatorError> {
        if self.0.last_update_epoch == clock.epoch {
            Ok(())
        } else {
            Err(SplCalculatorError::PoolNotUpdated)
        }
    }

    pub const fn lst_to_lamports_ratio(&self) -> U64RatioFloor<u64, u64> {
        let SplStakePool {
            total_lamports,
            pool_token_supply,
            ..
        } = self.0;
        U64RatioFloor {
            num: total_lamports,
            denom: pool_token_supply,
        }
    }

    pub const fn stake_withdrawal_fee(&self) -> U64FeeFloor<u64, u64> {
        let SplStakePool {
            stake_withdrawal_fee,
            ..
        } = &self.0;
        let Fee {
            denominator,
            numerator,
        } = stake_withdrawal_fee;
        U64FeeFloor {
            fee_num: *numerator,
            fee_denom: *denominator,
        }
    }
}

/// Assumes:
/// - stake pool manager is always valid, so stake withdraw fee will always be charged
/// - stake pool always has active and transient stake, so withdraw_source != StakeWithdrawSource::ValidatorRemoval
/// - stake pool has been updated for this epoch
impl SolValueCalculator for SplStakePoolCalc {
    fn calc_lst_to_sol(&self, pool_tokens: u64) -> Result<u64, ProgramError> {
        let pool_tokens_burnt = self.stake_withdrawal_fee().apply(pool_tokens)?;
        let withdraw_lamports = self.lst_to_lamports_ratio().apply(pool_tokens_burnt)?;
        Ok(withdraw_lamports)
    }

    fn calc_sol_to_lst(&self, withdraw_lamports: u64) -> Result<u64, ProgramError> {
        let pool_tokens_burnt = self.lst_to_lamports_ratio().reverse(withdraw_lamports)?;
        let pool_tokens = self.stake_withdrawal_fee().reverse(pool_tokens_burnt)?;
        Ok(pool_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use spl_calculator_interface::{AccountType, FutureEpochFee, Lockup};

    prop_compose! {
        fn fee_rate_lte_one()
            (denominator in any::<u64>())
            (numerator in 0..=denominator, denominator in Just(denominator)) -> Fee {
                Fee { denominator, numerator }
            }
    }

    prop_compose! {
        fn spl_stake_pool_calc()
            (stake_withdrawal_fee in fee_rate_lte_one(), total_lamports: u64, pool_token_supply: u64) -> SplStakePoolCalc {
                let zero_fee = Fee { numerator: 0, denominator: 0 };
                SplStakePoolCalc(
                    SplStakePool {
                        total_lamports,
                        pool_token_supply,
                        stake_withdrawal_fee,
                        account_type: AccountType::StakePool,
                        manager: Default::default(),
                        staker: Default::default(),
                        stake_deposit_authority: Default::default(),
                        stake_withdraw_bump_seed: Default::default(),
                        validator_list: Default::default(),
                        reserve_stake: Default::default(),
                        pool_mint: Default::default(),
                        manager_fee_account: Default::default(),
                        token_program_id: Default::default(),
                        last_update_epoch: Default::default(),
                        lockup: Lockup {
                            unix_timestamp: Default::default(),
                            epoch: Default::default(),
                            custodian: Default::default(),
                        },
                        epoch_fee: zero_fee.clone(),
                        next_epoch_fee: FutureEpochFee::None,
                        preferred_deposit_validator_vote_address: Default::default(),
                        preferred_withdraw_validator_vote_address: Default::default(),
                        stake_deposit_fee: zero_fee.clone(),
                        next_stake_withdrawal_fee: FutureEpochFee::None,
                        stake_referral_fee: Default::default(),
                        sol_deposit_authority: Default::default(),
                        sol_deposit_fee: zero_fee.clone(),
                        sol_referral_fee: Default::default(),
                        sol_withdraw_authority: Default::default(),
                        sol_withdrawal_fee: zero_fee,
                        next_sol_withdrawal_fee: FutureEpochFee::None,
                        last_epoch_pool_token_supply: Default::default(),
                        last_epoch_total_lamports: Default::default()
                    }
                )
            }
    }

    prop_compose! {
        fn spl_stake_pool_and_lst_amount()
            (calc in spl_stake_pool_calc())
            (pool_token in 0..=calc.0.pool_token_supply, calc in Just(calc)) -> (u64, SplStakePoolCalc) {
                (pool_token, calc)
            }
    }

    proptest! {
        #[test]
        fn lst_sol_round_trip((pool_tokens, calc) in spl_stake_pool_and_lst_amount()) {
            let withdraw_lamports = calc.calc_lst_to_sol(pool_tokens).unwrap();
            let withdraw_lamports_after = calc.calc_lst_to_sol(calc.calc_sol_to_lst(withdraw_lamports).unwrap()).unwrap();
            prop_assert_eq!(withdraw_lamports, withdraw_lamports_after)
        }
    }
}
