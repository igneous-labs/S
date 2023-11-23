use std::marker::PhantomData;

use bytemuck::try_from_bytes;
use generic_pool_calculator_interface::{CalculatorState, SetManagerKeys};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::GenericPoolSolValCalc;

pub struct SetManagerRootAccounts<P: GenericPoolSolValCalc, S: KeyedAccount + ReadonlyAccountData> {
    pub new_manager: Pubkey,
    pub state: S,
    _phantom: PhantomData<P>,
}

impl<P: GenericPoolSolValCalc, S: KeyedAccount + ReadonlyAccountData> SetManagerRootAccounts<P, S> {
    pub fn resolve(self) -> Result<SetManagerKeys, ProgramError> {
        if *self.state.key() != P::CALCULATOR_STATE_PDA {
            return Err(ProgramError::InvalidArgument);
        }
        let bytes = &self.state.data();
        let calc_state: &CalculatorState =
            try_from_bytes(bytes).map_err(|_e| ProgramError::InvalidAccountData)?;
        Ok(SetManagerKeys {
            new_manager: self.new_manager,
            manager: calc_state.manager,
            state: P::CALCULATOR_STATE_PDA,
        })
    }
}
