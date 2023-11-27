use borsh::BorshDeserialize;
use generic_pool_calculator_interface::GenericPoolCalculatorError;
use generic_pool_calculator_lib::account_resolvers::{
    LstSolCommonIntermediateAccounts, LstSolCommonIntermediateKeys,
};
use solana_readonly_account::{KeyedAccount, ReadonlyAccountData, ReadonlyAccountOwner};
use spl_calculator_interface::{AccountType, SplStakePool};
use spl_stake_pool_keys::spl_stake_pool_program;

fn deserialize_spl_stake_pool_checked<S: ReadonlyAccountData + ReadonlyAccountOwner>(
    spl_stake_pool: S,
) -> Result<SplStakePool, GenericPoolCalculatorError> {
    if *spl_stake_pool.owner() != spl_stake_pool_program::ID {
        return Err(GenericPoolCalculatorError::InvalidStakePoolProgramData);
    }
    let stake_pool = SplStakePool::deserialize(&mut spl_stake_pool.data().as_ref())
        .map_err(|_e| GenericPoolCalculatorError::InvalidStakePoolProgramData)?;
    if stake_pool.account_type != AccountType::StakePool {
        return Err(GenericPoolCalculatorError::InvalidStakePoolProgramData);
    }
    Ok(stake_pool)
}

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
    ) -> Result<(LstSolCommonIntermediateAccounts<Q>, SplStakePool), GenericPoolCalculatorError>
    {
        if *self.spl_stake_pool_prog.key() != spl_stake_pool_program::ID {
            return Err(GenericPoolCalculatorError::WrongPoolProgram);
        }
        let stake_pool = deserialize_spl_stake_pool_checked(&self.spl_stake_pool)?;
        Ok((
            LstSolCommonIntermediateAccounts {
                lst: stake_pool.pool_mint,
                pool_state: *self.spl_stake_pool.key(),
                pool_program: self.spl_stake_pool_prog,
            },
            stake_pool,
        ))
    }
}

/// Struct that uses defined const for POOL_PROGRAM_PROGDATA
/// so that it can be used without fetching POOL_PROGRAM
pub struct SplLstSolCommonRootAccountsConst<
    S: KeyedAccount + ReadonlyAccountData + ReadonlyAccountOwner,
> {
    pub spl_stake_pool: S,
}

impl<S: KeyedAccount + ReadonlyAccountData + ReadonlyAccountOwner>
    SplLstSolCommonRootAccountsConst<S>
{
    pub fn resolve(self) -> Result<LstSolCommonIntermediateKeys, GenericPoolCalculatorError> {
        let stake_pool = deserialize_spl_stake_pool_checked(&self.spl_stake_pool)?;
        Ok(LstSolCommonIntermediateKeys {
            lst: stake_pool.pool_mint,
            pool_state: *self.spl_stake_pool.key(),
        })
    }
}
