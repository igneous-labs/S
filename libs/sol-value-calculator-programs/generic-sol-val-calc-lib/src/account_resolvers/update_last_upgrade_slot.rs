use std::marker::PhantomData;

use bytemuck::try_from_bytes;
use generic_pool_calculator_interface::{CalculatorState, UpdateLastUpgradeSlotKeys};
use solana_program::{bpf_loader_upgradeable::UpgradeableLoaderState, program_error::ProgramError};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::GenericPoolSolValCalc;

pub struct UpdateLastUpgradeSlotRootAccounts<
    P: GenericPoolSolValCalc,
    S: KeyedAccount + ReadonlyAccountData,
    Q: KeyedAccount + ReadonlyAccountData,
> {
    pub state: S,
    pub pool_program: Q,
    _phantom: PhantomData<P>,
}

impl<
        P: GenericPoolSolValCalc,
        S: KeyedAccount + ReadonlyAccountData,
        Q: KeyedAccount + ReadonlyAccountData,
    > UpdateLastUpgradeSlotRootAccounts<P, S, Q>
{
    pub fn resolve(self) -> Result<UpdateLastUpgradeSlotKeys, ProgramError> {
        if *self.state.key() != P::CALCULATOR_STATE_PDA {
            return Err(ProgramError::InvalidArgument);
        }
        if *self.pool_program.key() != P::POOL_PROGRAM_ID {
            return Err(ProgramError::InvalidArgument);
        }

        let state_bytes = &self.state.data();
        let calc_state: &CalculatorState =
            try_from_bytes(state_bytes).map_err(|_e| ProgramError::InvalidAccountData)?;

        let pool_prog_bytes = &self.pool_program.data();
        let prog: UpgradeableLoaderState =
            bincode::deserialize(pool_prog_bytes).map_err(|_e| ProgramError::InvalidAccountData)?;
        let pool_program_data = if let UpgradeableLoaderState::Program {
            programdata_address,
        } = prog
        {
            programdata_address
        } else {
            return Err(ProgramError::InvalidAccountData);
        };

        Ok(UpdateLastUpgradeSlotKeys {
            manager: calc_state.manager,
            state: P::CALCULATOR_STATE_PDA,
            pool_program: P::POOL_PROGRAM_ID,
            pool_program_data,
        })
    }
}
