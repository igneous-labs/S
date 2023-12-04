use marinade_calculator_interface::{MarinadeCalculatorError, MarinadeState};
use sanctum_token_ratio::{AmtsAfterFee, U64FeeFloor, U64RatioFloor};
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::program_error::ProgramError;

#[derive(Debug, Clone)]
pub struct MarinadeStateCalc(pub MarinadeState);

pub const MAX_BP_CENTS: u32 = 1_000_000;

/// Reference
/// https://github.com/marinade-finance/liquid-staking-program/blob/26147376b75d8c971963da458623e646f2795e15/programs/marinade-finance/src/state/mod.rs#L96
/// TODO: check disabled
impl MarinadeStateCalc {
    pub const fn verify_marinade_not_paused(&self) -> Result<(), MarinadeCalculatorError> {
        if self.0.paused {
            Err(MarinadeCalculatorError::MarinadePaused)
        } else {
            Ok(())
        }
    }

    pub const fn total_cooling_down(&self) -> u64 {
        self.0.stake_system.delayed_unstake_cooling_down + self.0.emergency_cooling_down
    }

    pub const fn total_lamports_under_control(&self) -> u64 {
        self.0.validator_system.total_active_balance
            + self.total_cooling_down()
            + self.0.available_reserve_balance
    }

    pub const fn total_virtual_staked_lamports(&self) -> u64 {
        self.total_lamports_under_control()
            .saturating_sub(self.0.circulating_ticket_balance)
    }

    pub const fn msol_to_sol_ratio(&self) -> U64RatioFloor<u64, u64> {
        U64RatioFloor {
            num: self.total_virtual_staked_lamports(),
            denom: self.0.msol_supply,
        }
    }

    pub const fn delayed_unstake_fee(&self) -> U64FeeFloor<u32, u32> {
        U64FeeFloor {
            fee_num: self.0.delayed_unstake_fee.bp_cents,
            fee_denom: MAX_BP_CENTS,
        }
    }
}

impl SolValueCalculator for MarinadeStateCalc {
    fn calc_lst_to_sol(&self, msol_amount: u64) -> Result<u64, ProgramError> {
        let sol_value_of_msol_burned = self.msol_to_sol_ratio().apply(msol_amount)?;
        let AmtsAfterFee {
            amt_after_fee: lamports_for_user,
            ..
        } = self.delayed_unstake_fee().apply(sol_value_of_msol_burned)?;
        Ok(lamports_for_user)
    }

    fn calc_sol_to_lst(&self, lamports_for_user: u64) -> Result<u64, ProgramError> {
        let sol_value_of_msol_burned = self
            .delayed_unstake_fee()
            .pseudo_reverse(lamports_for_user)?;
        let msol_amount = self
            .msol_to_sol_ratio()
            .pseudo_reverse(sol_value_of_msol_burned)?;
        Ok(msol_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use marinade_calculator_interface::{
        Fee, FeeCents, LiqPool, List, StakeSystem, ValidatorSystem,
    };
    use proptest::prelude::*;
    use test_utils::prop_assert_diff_at_most;

    prop_compose! {
        fn total_cooling_down()
            (delayed_unstake_cooling_down in any::<u64>())
            (
                emergency_cooling_down in 0..=u64::MAX - delayed_unstake_cooling_down,
                delayed_unstake_cooling_down in Just(delayed_unstake_cooling_down)
            ) -> (u64, u64) {
                (emergency_cooling_down, delayed_unstake_cooling_down)
            }
    }

    prop_compose! {
        fn total_active_cooling_down()
            ((emergency_cooling_down, delayed_unstake_cooling_down) in total_cooling_down())
            (
                total_active_balance in 0..=u64::MAX - emergency_cooling_down - delayed_unstake_cooling_down,
                emergency_cooling_down in Just(emergency_cooling_down),
                delayed_unstake_cooling_down in Just(delayed_unstake_cooling_down)
            ) -> (u64, u64, u64) {
                (total_active_balance, emergency_cooling_down, delayed_unstake_cooling_down)
            }
    }

    prop_compose! {
        fn total_lamports_under_control()
            ((total_active_balance, emergency_cooling_down, delayed_unstake_cooling_down) in total_active_cooling_down())
            (
                available_reserve_balance in 0..=u64::MAX - total_active_balance - emergency_cooling_down - delayed_unstake_cooling_down,
                total_active_balance in Just(total_active_balance),
                emergency_cooling_down in Just(emergency_cooling_down),
                delayed_unstake_cooling_down in Just(delayed_unstake_cooling_down),
            ) -> (u64, u64, u64, u64) {
                (available_reserve_balance, total_active_balance, emergency_cooling_down, delayed_unstake_cooling_down)
            }
    }

    prop_compose! {
        fn marinade_calc()
            (
                (
                    available_reserve_balance,
                    total_active_balance,
                    emergency_cooling_down,
                    delayed_unstake_cooling_down,
                ) in total_lamports_under_control(),
                circulating_ticket_balance: u64,
                msol_supply: u64,
                bp_cents in 0..=MAX_BP_CENTS
            ) -> MarinadeStateCalc {
                let zero_fee = Fee { basis_points: Default::default() };
                let default_list = List {
                    account: Default::default(),
                    item_size: Default::default(),
                    count: Default::default(),
                    reserved1: Default::default(),
                    reserved2: Default::default()
                };

                MarinadeStateCalc(MarinadeState {
                    available_reserve_balance,
                    validator_system: ValidatorSystem {
                        total_active_balance,
                        // dont care
                        validator_list: default_list.clone(),
                        manager_authority: Default::default(),
                        total_validator_score: Default::default(),
                        auto_add_validator_enabled: Default::default(),
                    },
                    emergency_cooling_down,
                    stake_system: StakeSystem {
                        delayed_unstake_cooling_down,
                        // dont care
                        stake_list: default_list,
                        stake_deposit_bump_seed: Default::default(),
                        stake_withdraw_bump_seed: Default::default(),
                        slots_for_stake_delta: Default::default(),
                        last_stake_delta_epoch: Default::default(),
                        min_stake: Default::default(),
                        extra_stake_delta_runs:  Default::default()
                    },
                    circulating_ticket_balance,
                    msol_supply,
                    delayed_unstake_fee: FeeCents { bp_cents },
                    // dont care
                    discriminant: Default::default(),
                    msol_mint: Default::default(),
                    admin_authority: Default::default(),
                    operational_sol_account: Default::default(),
                    treasury_msol_account: Default::default(),
                    reserve_bump_seed: Default::default(),
                    msol_mint_authority_bump_seed: Default::default(),
                    rent_exempt_for_token_acc: Default::default(),
                    reward_fee: zero_fee.clone(),
                    liq_pool: LiqPool {
                        lp_mint: Default::default(),
                        lp_mint_authority_bump_seed: Default::default(),
                        sol_leg_bump_seed: Default::default(),
                        msol_leg_authority_bump_seed: Default::default(),
                        msol_leg: Default::default(),
                        lp_liquidity_target: Default::default(),
                        lp_max_fee: zero_fee.clone(),
                        lp_min_fee: zero_fee.clone(),
                        treasury_cut: zero_fee.clone(),
                        lp_supply: Default::default(),
                        lent_from_sol_leg: Default::default(),
                        liquidity_sol_cap: Default::default(),
                    },
                    msol_price: Default::default(),
                    circulating_ticket_count: Default::default(),
                    lent_from_reserve: Default::default(),
                    min_deposit: Default::default(),
                    min_withdraw: Default::default(),
                    staking_sol_cap: Default::default(),
                    pause_authority: Default::default(),
                    paused: Default::default(),
                    withdraw_stake_account_fee: FeeCents { bp_cents: Default::default() },
                    withdraw_stake_account_enabled: Default::default(),
                    last_stake_move_epoch: Default::default(),
                    stake_moved: Default::default(),
                    max_stake_moved_per_epoch: zero_fee,
                })
            }
    }

    prop_compose! {
        fn marinade_state_and_lst_amount()
            (calc in marinade_calc())
            (msol_amount in 0..=calc.0.msol_supply, calc in Just(calc)) -> (u64, MarinadeStateCalc) {
                (msol_amount, calc)
            }
    }

    proptest! {
        #[test]
        fn lst_sol_round_trip((msol_amount, calc) in marinade_state_and_lst_amount()) {
            let lamports_for_user = calc.calc_lst_to_sol(msol_amount).unwrap();
            let lamports_for_user_after = calc.calc_lst_to_sol(calc.calc_sol_to_lst(lamports_for_user).unwrap()).unwrap();
            // TODO: figure out this off-by-one, is this rly just the error of at most 1?
            prop_assert_diff_at_most!(lamports_for_user, lamports_for_user_after, 1);
        }
    }
}
