use generic_pool_calculator_interface::GenericPoolCalculatorError;
use marinade_calculator_interface::{FeeCents, MarinadeState};
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::program_error::ProgramError;

#[derive(Debug, Clone)]
pub struct MarinadeStateCalc(pub MarinadeState);

pub const MAX_BP_CENTS: u64 = 1_000_000;

/// Most fns copied from
/// https://github.com/marinade-finance/liquid-staking-program/blob/26147376b75d8c971963da458623e646f2795e15/programs/marinade-finance/src/state/mod.rs#L96
impl MarinadeStateCalc {
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

    pub fn msol_to_sol(&self, msol_amount: u64) -> Result<u64, GenericPoolCalculatorError> {
        if self.0.msol_supply == 0 {
            return Ok(0);
        }
        let res = (msol_amount as u128 * self.total_virtual_staked_lamports() as u128)
            / self.0.msol_supply as u128;
        u64::try_from(res).map_err(|_e| GenericPoolCalculatorError::MathError)
    }
}

/// Copied from:
/// https://github.com/marinade-finance/liquid-staking-program/blob/26147376b75d8c971963da458623e646f2795e15/programs/marinade-finance/src/state/fee.rs#L43
pub fn apply_fee(FeeCents { bp_cents }: &FeeCents, amount: u64) -> u64 {
    (*bp_cents as u128 * amount as u128 / MAX_BP_CENTS as u128) as u64
}

impl SolValueCalculator for MarinadeStateCalc {
    fn calc_lst_to_sol(&self, msol_amount: u64) -> Result<u64, ProgramError> {
        let sol_value_of_msol_burned = self.msol_to_sol(msol_amount)?;
        let delay_unstake_fee_lamports =
            apply_fee(&self.0.delayed_unstake_fee, sol_value_of_msol_burned);
        let lamports_for_user = sol_value_of_msol_burned - delay_unstake_fee_lamports;
        Ok(lamports_for_user)
    }

    fn calc_sol_to_lst(&self, _lamports_amount: u64) -> Result<u64, ProgramError> {
        todo!()
    }
}
