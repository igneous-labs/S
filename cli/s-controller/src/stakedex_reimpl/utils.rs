use std::error::Error;

use stakedex_sdk_common::{DepositStakeQuote, WithdrawStakeQuote};

use super::{DepositStakeStakedex, WithdrawStakeStakedex};

pub fn first_avail_withdraw_deposit_stake_quote(
    withdraw_amount: u64,
    withdraw: &WithdrawStakeStakedex,
    deposit: &DepositStakeStakedex,
) -> Result<(WithdrawStakeQuote, DepositStakeQuote), Box<dyn Error + Send + Sync + 'static>> {
    let iter = withdraw.withdraw_stake_quote_iter_dyn(withdraw_amount);
    for wsq in iter {
        let dsq = deposit.quote_deposit_stake(wsq);
        if !dsq.is_zero_out() {
            return Ok((wsq, dsq));
        }
    }
    Err("No route found".into())
}
