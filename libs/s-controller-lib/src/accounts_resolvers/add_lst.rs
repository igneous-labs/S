use s_controller_interface::{AddLstKeys, SControllerError};
use solana_program::{pubkey::Pubkey, system_program};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    find_lst_state_list_address, find_pool_reserves_address_with_pool_state_id,
    find_pool_state_address, find_protocol_fee_accumulator_address_with_protocol_fee_id,
    find_protocol_fee_address,
    program::{LST_STATE_LIST_ID, POOL_STATE_ID, PROTOCOL_FEE_ID},
    try_pool_state, FindLstPdaAtaKeys,
};

#[derive(Clone, Copy, Debug)]
pub struct LstStateBumps {
    pub protocol_fee_accumulator: u8,
    pub pool_reserves: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct AddLstFreeArgs<
    S: ReadonlyAccountData + ReadonlyAccountPubkey,
    M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
> {
    pub payer: Pubkey,
    pub sol_value_calculator: Pubkey,
    pub pool_state: S,
    pub lst_mint: M,
}

struct ResolveInner {
    pool_state: Pubkey,
    protocol_fee_accumulator_auth: Pubkey,
    lst_state_list: Pubkey,
}

impl<
        S: ReadonlyAccountData + ReadonlyAccountPubkey,
        M: ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > AddLstFreeArgs<S, M>
{
    pub fn resolve(self) -> Result<(AddLstKeys, LstStateBumps), SControllerError> {
        self.resolve_inner(ResolveInner {
            pool_state: POOL_STATE_ID,
            protocol_fee_accumulator_auth: PROTOCOL_FEE_ID,
            lst_state_list: LST_STATE_LIST_ID,
        })
    }

    pub fn resolve_for_prog(
        self,
        program_id: Pubkey,
    ) -> Result<(AddLstKeys, LstStateBumps), SControllerError> {
        self.resolve_inner(ResolveInner {
            pool_state: find_pool_state_address(program_id).0,
            protocol_fee_accumulator_auth: find_protocol_fee_address(program_id).0,
            lst_state_list: find_lst_state_list_address(program_id).0,
        })
    }

    fn resolve_inner(
        self,
        ResolveInner {
            pool_state,
            protocol_fee_accumulator_auth,
            lst_state_list,
        }: ResolveInner,
    ) -> Result<(AddLstKeys, LstStateBumps), SControllerError> {
        let AddLstFreeArgs {
            payer,
            sol_value_calculator,
            pool_state: pool_state_acc,
            lst_mint,
        } = self;

        if *pool_state_acc.pubkey() != pool_state {
            return Err(SControllerError::IncorrectPoolState);
        }

        let pool_state_data = pool_state_acc.data();
        let pool_state_data = try_pool_state(&pool_state_data)?;

        let find_pda_keys = FindLstPdaAtaKeys {
            lst_mint: *lst_mint.pubkey(),
            token_program: *lst_mint.owner(),
        };
        let (pool_reserves, pool_reserves_bump) =
            find_pool_reserves_address_with_pool_state_id(pool_state, find_pda_keys);
        let (protocol_fee_accumulator, protocol_fee_accumulator_bump) =
            find_protocol_fee_accumulator_address_with_protocol_fee_id(
                protocol_fee_accumulator_auth,
                find_pda_keys,
            );

        Ok((
            AddLstKeys {
                payer,
                sol_value_calculator,
                lst_mint: *lst_mint.pubkey(),
                admin: pool_state_data.admin,
                pool_reserves,
                protocol_fee_accumulator,
                protocol_fee_accumulator_auth,
                pool_state,
                lst_state_list,
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
