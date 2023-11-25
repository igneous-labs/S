use generic_pool_calculator_interface::{GenericPoolCalculatorError, UpdateLastUpgradeSlotKeys};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{
    utils::{read_programdata_addr, try_calculator_state},
    GenericPoolSolValCalc,
};

pub struct UpdateLastUpgradeSlotRootAccounts<
    S: KeyedAccount + ReadonlyAccountData,
    Q: KeyedAccount + ReadonlyAccountData,
> {
    pub state: S,
    pub pool_program: Q,
}

impl<S: KeyedAccount + ReadonlyAccountData, Q: KeyedAccount + ReadonlyAccountData>
    UpdateLastUpgradeSlotRootAccounts<S, Q>
{
    pub fn resolve<P: GenericPoolSolValCalc>(
        self,
    ) -> Result<UpdateLastUpgradeSlotKeys, GenericPoolCalculatorError> {
        if *self.state.key() != P::CALCULATOR_STATE_PDA {
            return Err(GenericPoolCalculatorError::WrongCalculatorStatePda);
        }
        if *self.pool_program.key() != P::POOL_PROGRAM_ID {
            return Err(GenericPoolCalculatorError::WrongPoolProgram);
        }

        let state_bytes = &self.state.data();
        let calc_state = try_calculator_state(state_bytes)?;

        let pool_program_data = read_programdata_addr(&self.pool_program)?;

        Ok(UpdateLastUpgradeSlotKeys {
            manager: calc_state.manager,
            state: P::CALCULATOR_STATE_PDA,
            pool_program: P::POOL_PROGRAM_ID,
            pool_program_data,
        })
    }
}
