use std::marker::PhantomData;

use generic_pool_calculator_interface::InitKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::GenericPoolSolValCalc;

pub struct InitRootAccounts<P: GenericPoolSolValCalc> {
    pub payer: Pubkey,

    /// Associate generic with struct so
    /// that InitRootAccounts<Spl> is different type from
    /// InitRootAccounts<Marinade>
    _phantom: PhantomData<P>,
}

impl<P: GenericPoolSolValCalc> InitRootAccounts<P> {
    pub fn resolve(self) -> InitKeys {
        InitKeys {
            payer: self.payer,
            state: P::CALCULATOR_STATE_PDA,
            system_program: system_program::ID,
        }
    }
}
