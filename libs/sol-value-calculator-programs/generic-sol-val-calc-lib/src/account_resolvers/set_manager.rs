use std::marker::PhantomData;

use bytemuck::try_from_bytes;
use generic_pool_calculator_interface::{
    CalculatorState, GenericPoolCalculatorError, SetManagerKeys,
};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::GenericPoolSolValCalc;

pub struct SetManagerRootAccounts<P: GenericPoolSolValCalc, S: KeyedAccount + ReadonlyAccountData> {
    pub new_manager: Pubkey,
    pub state: S,
    _phantom: PhantomData<P>,
}

impl<P: GenericPoolSolValCalc, S: KeyedAccount + ReadonlyAccountData> SetManagerRootAccounts<P, S> {
    pub fn resolve(self) -> Result<SetManagerKeys, GenericPoolCalculatorError> {
        if *self.state.key() != P::CALCULATOR_STATE_PDA {
            return Err(GenericPoolCalculatorError::WrongCalculatorStatePda);
        }
        let bytes = &self.state.data();
        let calc_state: &CalculatorState = try_from_bytes(bytes)
            .map_err(|_e| GenericPoolCalculatorError::InvalidCalculatorStateData)?;
        Ok(SetManagerKeys {
            new_manager: self.new_manager,
            manager: calc_state.manager,
            state: P::CALCULATOR_STATE_PDA,
        })
    }
}
