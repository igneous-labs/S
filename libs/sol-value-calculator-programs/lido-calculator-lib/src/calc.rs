use lido_calculator_interface::{ExchangeRate, Lido, LidoCalculatorError};
use sanctum_token_ratio::U64RatioFloor;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{clock::Clock, program_error::ProgramError};

#[derive(Debug, Clone)]
pub struct LidoCalc(pub Lido);

/// Reference
/// https://github.com/lidofinance/solido/blob/4e071bed845fca1e30215ec4e0be6b63e659bc18/program/src/processor.rs#L1034
impl LidoCalc {
    pub const fn verify_pool_updated_for_this_epoch(
        &self,
        clock: &Clock,
    ) -> Result<(), LidoCalculatorError> {
        if self.0.exchange_rate.computed_in_epoch < clock.epoch {
            return Err(LidoCalculatorError::ExchangeRateNotUpdatedInThisEpoch);
        }
        Ok(())
    }

    pub const fn stlamports_to_lamports_ratio(&self) -> U64RatioFloor<u64, u64> {
        let Lido {
            exchange_rate:
                ExchangeRate {
                    st_sol_supply,
                    sol_balance,
                    ..
                },
            ..
        } = self.0;
        U64RatioFloor {
            num: sol_balance,
            denom: st_sol_supply,
        }
    }
}

/// Assumes:
/// - stake pool manager is always valid, so stake withdraw fee will always be charged
/// - stake pool always has active and transient stake, so withdraw_source != StakeWithdrawSource::ValidatorRemoval
/// - stake pool has been updated for this epoch
impl SolValueCalculator for LidoCalc {
    fn calc_lst_to_sol(&self, pool_tokens: u64) -> Result<u64, ProgramError> {
        Ok(self.stlamports_to_lamports_ratio().apply(pool_tokens)?)
    }

    fn calc_sol_to_lst(&self, withdraw_lamports: u64) -> Result<u64, ProgramError> {
        Ok(self
            .stlamports_to_lamports_ratio()
            .pseudo_reverse(withdraw_lamports)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lido_calculator_interface::{
        Criteria, FeeRecipients, LamportsHistogram, Metrics, RewardDistribution, WithdrawMetric,
    };
    use proptest::prelude::*;

    prop_compose! {
        fn lido_calc()
            (sol_balance: u64, st_sol_supply: u64) -> LidoCalc {
                LidoCalc(
                    Lido {
                        exchange_rate: ExchangeRate {
                            st_sol_supply,
                            sol_balance,
                            computed_in_epoch: Default::default()
                        },
                        account_type: Default::default(),
                        lido_version: Default::default(),
                        manager: Default::default(),
                        st_sol_mint: Default::default(),
                        sol_reserve_account_bump_seed: Default::default(),
                        stake_authority_bump_seed: Default::default(),
                        mint_authority_bump_seed: Default::default(),
                        reward_distribution: RewardDistribution {
                            treasury_fee: Default::default(),
                            developer_fee: Default::default(),
                            st_sol_appreciation: Default::default()
                        },
                        fee_recipients: FeeRecipients {
                            treasury_account: Default::default(),
                            developer_account: Default::default()
                        },
                        metrics: Metrics {
                            fee_treasury_total_lamports: Default::default(),
                            fee_validation_total_lamports: Default::default(),
                            fee_developer_total_lamports: Default::default(),
                            st_sol_appreciation_total_lamports: Default::default(),
                            fee_treasury_total_st_lamports: Default::default(),
                            fee_validation_total_st_lamports: Default::default(),
                            fee_developer_total_st_lamports: Default::default(),
                            deposit_amount: LamportsHistogram {
                                counts: Default::default(),
                                total: Default::default()
                            },
                            withdraw_amount: WithdrawMetric {
                                total_st_sol_amount: Default::default(),
                                total_sol_amount: Default::default(),
                                count: Default::default()
                            }
                        },
                        criteria: Criteria {
                            max_commission: Default::default(),
                            min_block_production_rate: Default::default(),
                            min_vote_success_rate: Default::default()
                        },
                        validator_list: Default::default(),
                        validator_perf_list: Default::default(),
                        maintainer_list: Default::default(),
                    }
                )
            }
    }

    prop_compose! {
        fn lido_calc_and_stsol_amt()
            (calc in lido_calc())
            (stsol_amt in 0..=calc.0.exchange_rate.st_sol_supply, calc in Just(calc)) -> (u64, LidoCalc) {
                (stsol_amt, calc)
            }
    }

    proptest! {
        #[test]
        fn lst_sol_round_trip((stsol_amt, calc) in lido_calc_and_stsol_amt()) {
            let withdraw_lamports = calc.calc_lst_to_sol(stsol_amt).unwrap();
            let withdraw_lamports_after = calc.calc_lst_to_sol(calc.calc_sol_to_lst(withdraw_lamports).unwrap()).unwrap();
            prop_assert_eq!(withdraw_lamports, withdraw_lamports_after)
        }
    }
}
