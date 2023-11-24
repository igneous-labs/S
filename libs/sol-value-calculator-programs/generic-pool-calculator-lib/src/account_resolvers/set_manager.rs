use std::marker::PhantomData;

use generic_pool_calculator_interface::{GenericPoolCalculatorError, SetManagerKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{utils::try_calculator_state, GenericPoolSolValCalc};

pub struct SetManagerRootAccounts<P: GenericPoolSolValCalc, S: KeyedAccount + ReadonlyAccountData> {
    pub new_manager: Pubkey,
    pub state: S,
    pub phantom: PhantomData<P>,
}

impl<P: GenericPoolSolValCalc, S: KeyedAccount + ReadonlyAccountData> SetManagerRootAccounts<P, S> {
    pub fn resolve(self) -> Result<SetManagerKeys, GenericPoolCalculatorError> {
        if *self.state.key() != P::CALCULATOR_STATE_PDA {
            return Err(GenericPoolCalculatorError::WrongCalculatorStatePda);
        }
        let bytes = &self.state.data();
        let calc_state = try_calculator_state(bytes)?;
        Ok(SetManagerKeys {
            new_manager: self.new_manager,
            manager: calc_state.manager,
            state: P::CALCULATOR_STATE_PDA,
        })
    }
}
