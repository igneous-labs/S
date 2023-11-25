use generic_pool_calculator_interface::GenericPoolCalculatorError;
use generic_pool_calculator_lib::utils::checked_div_ceil;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::program_error::ProgramError;
use spl_calculator_interface::{Fee, SplStakePool};

pub struct SplStakePoolCalc(pub SplStakePool);

/// Assumes:
/// - stake pool manager is always valid, so stake withdraw fee will always be charged
/// - stake pool always has active and transient stake, so withdraw_source != StakeWithdrawSource::ValidatorRemoval
impl SolValueCalculator for SplStakePoolCalc {
    fn calc_lst_to_sol(&self, pool_tokens: u64) -> Result<u64, ProgramError> {
        let pool = &self.0;

        // Copied from:
        // https://github.com/solana-labs/solana-program-library/blob/52db94edb5571309dd7e5472c530ff56bcc30ae5/stake-pool/program/src/processor.rs#L3169-L3184
        let pool_tokens_fee = apply_fee(&pool.stake_withdrawal_fee, pool_tokens)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        let pool_tokens_burnt = pool_tokens
            .checked_sub(pool_tokens_fee)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        let withdraw_lamports = calc_lamports_withdraw_amount(pool, pool_tokens_burnt)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        Ok(withdraw_lamports)
    }

    /// TODO: make a prop-test for this: calc_sol_to_lst(calc_lst_to_sol(amt)) = amt
    fn calc_sol_to_lst(&self, withdraw_lamports: u64) -> Result<u64, ProgramError> {
        let pool = &self.0;

        let pool_tokens_burnt = calc_pool_tokens_burnt(pool, withdraw_lamports)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        let pool_tokens = reverse_fee(&pool.stake_withdrawal_fee, pool_tokens_burnt)
            .ok_or(GenericPoolCalculatorError::MathError)?;
        Ok(pool_tokens)
    }
}

/// Copied from:
/// https://github.com/solana-labs/solana-program-library/blob/52db94edb5571309dd7e5472c530ff56bcc30ae5/stake-pool/program/src/state.rs#L941
/// Returns the amount to collect in fees (amt * numerator / denominator)
pub fn apply_fee(fee: &Fee, amt: u64) -> Option<u64> {
    if fee.denominator == 0 {
        return Some(0);
    }
    let res = (amt as u128)
        .checked_mul(fee.numerator as u128)?
        .checked_div(fee.denominator as u128)?;
    res.try_into().ok()
}

/// Inverse of [`apply_fee`], returns the amount before fees
/// TODO: make prop test for reverse_fee(apply_fee(amt)) = amt
pub fn reverse_fee(fee: &Fee, amt_after_deduct_fee: u64) -> Option<u64> {
    if fee.denominator == 0 || fee.numerator == 0 {
        return Some(amt_after_deduct_fee);
    }
    let res = (amt_after_deduct_fee as u128)
        .checked_mul(fee.denominator as u128)?
        .checked_div(fee.denominator.checked_sub(fee.numerator)? as u128)?;
    res.try_into().ok()
}

/// Copied from:
/// https://github.com/solana-labs/solana-program-library/blob/52db94edb5571309dd7e5472c530ff56bcc30ae5/stake-pool/program/src/state.rs#L178
pub fn calc_lamports_withdraw_amount(
    SplStakePool {
        total_lamports,
        pool_token_supply,
        ..
    }: &SplStakePool,
    pool_tokens: u64,
) -> Option<u64> {
    let numerator = (pool_tokens as u128).checked_mul(*total_lamports as u128)?;
    let denominator = *pool_token_supply as u128;
    if numerator < denominator || denominator == 0 {
        Some(0)
    } else {
        u64::try_from(numerator.checked_div(denominator)?).ok()
    }
}

/// Inverse of [`calc_lamports_withdraw_amount`]
/// TODO: make prop test for calc_pool_tokens_burnt(calc_lamports_withdraw_amount(amt)) = amt
pub fn calc_pool_tokens_burnt(
    SplStakePool {
        total_lamports,
        pool_token_supply,
        ..
    }: &SplStakePool,
    withdraw_lamports: u64,
) -> Option<u64> {
    if *total_lamports == 0 {
        return Some(0);
    }
    let pool_tokens_burnt = checked_div_ceil(
        (withdraw_lamports as u128).checked_mul(*pool_token_supply as u128)?,
        *total_lamports as u128,
    )?;
    pool_tokens_burnt.try_into().ok()
}
