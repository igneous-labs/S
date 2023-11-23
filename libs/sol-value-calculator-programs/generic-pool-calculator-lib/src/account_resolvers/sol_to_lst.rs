use std::marker::PhantomData;

use generic_pool_calculator_interface::{GenericPoolCalculatorError, SolToLstKeys};
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData};

use crate::{utils::read_programdata_addr, GenericPoolSolValCalc};

/// NB: This struct requires a impl-specific resolver to resolve to in order to derive
/// lst from pool_state and check them
pub struct SolToLstRootAccounts<P: GenericPoolSolValCalc, Q: KeyedAccount + ReadonlyAccountData> {
    pub lst: Pubkey,
    pub pool_state: Pubkey,
    pub pool_program: Q,
    pub phantom: PhantomData<P>,
}

impl<P: GenericPoolSolValCalc, Q: KeyedAccount + ReadonlyAccountData> SolToLstRootAccounts<P, Q> {
    pub fn resolve(self) -> Result<SolToLstKeys, GenericPoolCalculatorError> {
        if *self.pool_program.key() != P::POOL_PROGRAM_ID {
            return Err(GenericPoolCalculatorError::WrongPoolProgram);
        }
        Ok(SolToLstKeys {
            lst: self.lst,
            state: P::CALCULATOR_STATE_PDA,
            pool_state: self.pool_state,
            pool_program: *self.pool_program.key(),
            pool_program_data: read_programdata_addr(&self.pool_program)?,
        })
    }
}
