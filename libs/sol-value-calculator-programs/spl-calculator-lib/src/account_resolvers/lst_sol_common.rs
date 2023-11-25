use borsh::BorshDeserialize;
use generic_pool_calculator_interface::GenericPoolCalculatorError;
use generic_pool_calculator_lib::account_resolvers::LstSolCommonIntermediateAccounts;
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};
use spl_calculator_interface::{AccountType, SplStakePool};
use spl_stake_pool_keys::spl_stake_pool_program;

use crate::SplSolValCalc;

pub struct SplLstSolCommonRootAccounts<
    S: KeyedAccount + ReadonlyAccountData + ReadonlyAccountOwner,
    Q: KeyedAccount + ReadonlyAccountData,
> {
    pub spl_stake_pool: S,
    pub spl_stake_pool_prog: Q,
}

impl<
        S: KeyedAccount + ReadonlyAccountData + ReadonlyAccountOwner,
        Q: KeyedAccount + ReadonlyAccountData,
    > SplLstSolCommonRootAccounts<S, Q>
{
    pub fn resolve(
        self,
    ) -> Result<
        (
            LstSolCommonIntermediateAccounts<SplSolValCalc, Q>,
            SplStakePool,
        ),
        GenericPoolCalculatorError,
    > {
        if *self.spl_stake_pool_prog.key() != spl_stake_pool_program::ID {
            return Err(GenericPoolCalculatorError::WrongPoolProgram);
        }
        if *self.spl_stake_pool.owner() != spl_stake_pool_program::ID {
            return Err(GenericPoolCalculatorError::InvalidStakePoolProgramData);
        }
        let stake_pool = SplStakePool::deserialize(&mut self.spl_stake_pool.data().as_ref())
            .map_err(|_e| GenericPoolCalculatorError::InvalidStakePoolProgramData)?;
        if stake_pool.account_type != AccountType::StakePool {
            return Err(GenericPoolCalculatorError::InvalidStakePoolProgramData);
        }
        Ok((
            LstSolCommonIntermediateAccounts {
                lst: stake_pool.pool_mint,
                pool_state: *self.spl_stake_pool.key(),
                pool_program: self.spl_stake_pool_prog,
                phantom: Default::default(),
            },
            stake_pool,
        ))
    }
}
