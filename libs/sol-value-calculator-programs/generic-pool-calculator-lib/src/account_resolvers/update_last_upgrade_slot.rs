use generic_pool_calculator_interface::{GenericPoolCalculatorError, UpdateLastUpgradeSlotKeys};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};

use crate::{
    utils::{read_programdata_addr, try_calculator_state},
    GenericPoolSolValCalc,
};

pub struct UpdateLastUpgradeSlotFreeArgs<
    S: ReadonlyAccountPubkey + ReadonlyAccountData,
    Q: ReadonlyAccountPubkey + ReadonlyAccountData,
> {
    pub state: S,
    pub pool_program: Q,
}

impl<
        S: ReadonlyAccountPubkey + ReadonlyAccountData,
        Q: ReadonlyAccountPubkey + ReadonlyAccountData,
    > UpdateLastUpgradeSlotFreeArgs<S, Q>
{
    pub fn resolve<P: GenericPoolSolValCalc>(
        self,
    ) -> Result<UpdateLastUpgradeSlotKeys, GenericPoolCalculatorError> {
        if *self.state.pubkey() != P::CALCULATOR_STATE_PDA {
            return Err(GenericPoolCalculatorError::WrongCalculatorStatePda);
        }
        if *self.pool_program.pubkey() != P::POOL_PROGRAM_ID {
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

/// Struct that uses defined const for POOL_PROGRAM_PROGDATA
/// so that it can be used without fetching POOL_PROGRAM
pub struct UpdateLastUpgradeSlotFreeArgsConst<S: ReadonlyAccountPubkey + ReadonlyAccountData> {
    pub state: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData> UpdateLastUpgradeSlotFreeArgsConst<S> {
    pub fn resolve<P: GenericPoolSolValCalc>(
        self,
    ) -> Result<UpdateLastUpgradeSlotKeys, GenericPoolCalculatorError> {
        if *self.state.pubkey() != P::CALCULATOR_STATE_PDA {
            return Err(GenericPoolCalculatorError::WrongCalculatorStatePda);
        }

        let state_bytes = &self.state.data();
        let calc_state = try_calculator_state(state_bytes)?;

        Ok(UpdateLastUpgradeSlotKeys {
            manager: calc_state.manager,
            state: P::CALCULATOR_STATE_PDA,
            pool_program: P::POOL_PROGRAM_ID,
            pool_program_data: P::POOL_PROGRAM_PROGDATA_ID,
        })
    }
}
