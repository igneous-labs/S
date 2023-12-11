use generic_pool_calculator_interface::{GenericPoolCalculatorError, SetManagerKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{utils::try_calculator_state, GenericPoolSolValCalc};

pub struct SetManagerFreeArgs<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub new_manager: Pubkey,
    pub state: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> SetManagerFreeArgs<S> {
    pub fn resolve<P: GenericPoolSolValCalc>(
        self,
    ) -> Result<SetManagerKeys, GenericPoolCalculatorError> {
        if *self.state.pubkey() != P::CALCULATOR_STATE_PDA {
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
