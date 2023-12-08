use s_controller_interface::{SControllerError, WithdrawProtocolFeesKeys};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    find_protocol_fee_accumulator_address,
    program::{POOL_STATE_ID, PROTOCOL_FEE_ID},
    try_pool_state, FindLstPdaAtaKeys,
};

#[derive(Clone, Copy, Debug)]
pub struct WithdrawProtocolFeesFreeArgs<
    S: ReadonlyAccountData + KeyedAccount,
    W: ReadonlyAccountOwner + KeyedAccount,
> {
    pub pool_state: S,
    pub withdraw_to: W,
}

impl<S: ReadonlyAccountData + KeyedAccount, W: ReadonlyAccountOwner + KeyedAccount>
    WithdrawProtocolFeesFreeArgs<S, W>
{
    pub fn resolve(self) -> Result<WithdrawProtocolFeesKeys, SControllerError> {
        let WithdrawProtocolFeesFreeArgs {
            pool_state: pool_state_acc,
            withdraw_to,
        } = self;

        if *pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        let find_pda_keys = FindLstPdaAtaKeys {
            lst_mint: *withdraw_to.key(),
            token_program: *withdraw_to.owner(),
        };
        let (protocol_fee_accumulator, _protocol_fee_accumulator_bump) =
            find_protocol_fee_accumulator_address(find_pda_keys);

        Ok(WithdrawProtocolFeesKeys {
            pool_state: POOL_STATE_ID,
            protocol_fee_accumulator,
            protocol_fee_accumulator_auth: PROTOCOL_FEE_ID,
            protocol_fee_beneficiary: pool_state.protocol_fee_beneficiary,
            token_program: *withdraw_to.owner(),
            withdraw_to: *withdraw_to.key(),
        })
    }
}
