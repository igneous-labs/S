use std::error::Error;

use stakedex_sdk_common::{
    DepositStakeQuote, WithdrawStakeQuote, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
};

use super::{DepositStakeStakedex, WithdrawStakeStakedex};

pub fn first_avail_withdraw_deposit_stake_quote(
    withdraw_amount: u64,
    withdraw: &WithdrawStakeStakedex,
    deposit: &DepositStakeStakedex,
) -> Result<(WithdrawStakeQuote, DepositStakeQuote), Box<dyn Error + Send + Sync + 'static>> {
    let iter = withdraw.withdraw_stake_quote_iter_dyn(withdraw_amount);
    for wsq in iter {
        let wsq = prefund_transform_wsq(wsq);
        let dsq = deposit.quote_deposit_stake(wsq);
        if !dsq.is_zero_out() {
            return Ok((wsq, dsq));
        }
    }
    Err("No route found".into())
}

/// Since we're prefunding bridge stake with the rent, we need to add it to the output stake account
fn prefund_transform_wsq(mut wsq: WithdrawStakeQuote) -> WithdrawStakeQuote {
    wsq.lamports_staked = wsq.lamports_out;
    wsq.lamports_out += STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS;
    wsq
}
