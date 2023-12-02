use s_controller_interface::{AddLstKeys, SControllerError};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};

use crate::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID, PROTOCOL_FEE_ID},
    try_pool_state, FindLstAccountAddressKeys,
};

#[derive(Clone, Copy, Debug)]
pub struct LstStateBumps {
    pub protocol_fee_accumulator: u8,
    pub pool_reserves: u8,
}

pub struct AddLstFreeArgs<
    S: ReadonlyAccountData + KeyedAccount,
    M: ReadonlyAccountOwner + KeyedAccount,
> {
    pub payer: Pubkey,
    pub sol_value_calculator: Pubkey,
    pub pool_state: S,
    pub lst_mint: M,
}

impl<S: ReadonlyAccountData + KeyedAccount, M: ReadonlyAccountOwner + KeyedAccount>
    AddLstFreeArgs<S, M>
{
    pub fn resolve(self) -> Result<(AddLstKeys, LstStateBumps), SControllerError> {
        let AddLstFreeArgs {
            payer,
            sol_value_calculator,
            pool_state: pool_state_acc,
            lst_mint,
        } = self;

        if *pool_state_acc.key() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = pool_state_acc.data();
        let pool_state = try_pool_state(&pool_state_data)?;

        let find_pda_keys = FindLstAccountAddressKeys {
            lst_mint: *lst_mint.key(),
            token_program: *lst_mint.owner(),
        };
        let (pool_reserves, pool_reserves_bump) = find_pool_reserves_address(find_pda_keys);
        let (protocol_fee_accumulator, protocol_fee_accumulator_bump) =
            find_protocol_fee_accumulator_address(find_pda_keys);

        Ok((
            AddLstKeys {
                payer,
                sol_value_calculator,
                lst_mint: *lst_mint.key(),
                admin: pool_state.admin,
                pool_reserves,
                protocol_fee_accumulator,
                protocol_fee_accumulator_auth: PROTOCOL_FEE_ID,
                pool_state: POOL_STATE_ID,
                lst_state_list: LST_STATE_LIST_ID,
                associated_token_program: spl_associated_token_account::ID,
                system_program: system_program::ID,
                lst_token_program: *lst_mint.owner(),
            },
            LstStateBumps {
                protocol_fee_accumulator: protocol_fee_accumulator_bump,
                pool_reserves: pool_reserves_bump,
            },
        ))
    }
}
