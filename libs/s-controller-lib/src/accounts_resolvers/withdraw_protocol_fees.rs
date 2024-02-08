use s_controller_interface::{SControllerError, WithdrawProtocolFeesKeys};
use sanctum_token_lib::{token_account_mint, MintWithTokenProgram};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};

use crate::{
    find_pool_state_address, find_protocol_fee_accumulator_address,
    find_protocol_fee_accumulator_address_with_protocol_fee_id, find_protocol_fee_address,
    program::{POOL_STATE_ID, PROTOCOL_FEE_ID},
    try_pool_state, FindLstPdaAtaKeys,
};

#[derive(Clone, Copy, Debug)]
pub struct WithdrawProtocolFeesPdas {
    pub pool_state: Pubkey,
    pub protocol_fee_accumulator_auth: Pubkey,
    pub protocol_fee_accumulator: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct WithdrawProtocolFeesFreeArgs<S, W> {
    pub pool_state: S,
    pub withdraw_to: W,
}

impl<
        S: ReadonlyAccountData + ReadonlyAccountPubkey,
        W: ReadonlyAccountData + ReadonlyAccountOwner + ReadonlyAccountPubkey,
    > WithdrawProtocolFeesFreeArgs<S, W>
{
    pub fn resolve(self) -> Result<WithdrawProtocolFeesKeys, ProgramError> {
        let WithdrawProtocolFeesFreeArgs {
            pool_state,
            withdraw_to,
        } = self;

        if *pool_state.pubkey() != POOL_STATE_ID {
            return Err(SControllerError::IncorrectPoolState.into());
        }

        let lst_mint = token_account_mint(&withdraw_to)?;
        let (protocol_fee_accumulator, _protocol_fee_accumulator_bump) =
            find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
                lst_mint,
                token_program: *withdraw_to.owner(),
            });
        WithdrawProtocolFeesByMintFreeArgs {
            pool_state,
            withdraw_to: *withdraw_to.pubkey(),
            lst_mint: MintWithTokenProgram {
                pubkey: lst_mint,
                token_program: *withdraw_to.owner(),
            },
        }
        .resolve_with_pdas(WithdrawProtocolFeesPdas {
            pool_state: POOL_STATE_ID,
            protocol_fee_accumulator_auth: PROTOCOL_FEE_ID,
            protocol_fee_accumulator,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WithdrawProtocolFeesByMintFreeArgs<S, M> {
    pub pool_state: S,
    pub lst_mint: M,
    pub withdraw_to: Pubkey,
}

impl<S: ReadonlyAccountData, M: ReadonlyAccountOwner + ReadonlyAccountPubkey>
    WithdrawProtocolFeesByMintFreeArgs<S, M>
{
    pub fn resolve_for_prog(
        self,
        program_id: Pubkey,
    ) -> Result<WithdrawProtocolFeesKeys, ProgramError> {
        let protocol_fee_accumulator_auth = find_protocol_fee_address(program_id).0;
        let protocol_fee_accumulator = find_protocol_fee_accumulator_address_with_protocol_fee_id(
            protocol_fee_accumulator_auth,
            FindLstPdaAtaKeys {
                lst_mint: *self.lst_mint.pubkey(),
                token_program: *self.lst_mint.owner(),
            },
        )
        .0;
        self.resolve_with_pdas(WithdrawProtocolFeesPdas {
            pool_state: find_pool_state_address(program_id).0,
            protocol_fee_accumulator_auth,
            protocol_fee_accumulator,
        })
    }

    pub fn resolve_with_pdas(
        self,
        WithdrawProtocolFeesPdas {
            pool_state,
            protocol_fee_accumulator_auth,
            protocol_fee_accumulator,
        }: WithdrawProtocolFeesPdas,
    ) -> Result<WithdrawProtocolFeesKeys, ProgramError> {
        let Self {
            pool_state: pool_state_acc,
            withdraw_to,
            lst_mint,
        } = self;

        let pool_state_data = pool_state_acc.data();
        let protocol_fee_beneficiary = try_pool_state(&pool_state_data)?.protocol_fee_beneficiary;

        Ok(WithdrawProtocolFeesKeys {
            pool_state,
            protocol_fee_accumulator,
            protocol_fee_accumulator_auth,
            protocol_fee_beneficiary,
            withdraw_to,
            token_program: *lst_mint.owner(),
            lst_mint: *lst_mint.pubkey(),
        })
    }
}
