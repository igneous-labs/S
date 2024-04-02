use borsh::BorshDeserialize;
use generic_pool_calculator_interface::{GenericPoolCalculatorError, LST_TO_SOL_IX_ACCOUNTS_LEN};
use generic_pool_calculator_lib::{
    account_resolvers::{LstSolCommonIntermediateArgs, LstSolCommonIntermediateKeys},
    GenericPoolSolValCalc,
};
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};
use spl_calculator_interface::{AccountType, SplStakePool};

use crate::{SanctumSplMultiSolValCalc, SanctumSplSolValCalc, SplSolValCalc};

pub fn deserialize_spl_stake_pool_checked<S: ReadonlyAccountData + ReadonlyAccountOwner>(
    spl_stake_pool: S,
) -> Result<SplStakePool, GenericPoolCalculatorError> {
    deserialize_stake_pool_check_program_owner(spl_stake_pool, SplSolValCalc::POOL_PROGRAM_ID)
}

pub fn deserialize_sanctum_spl_stake_pool_checked<S: ReadonlyAccountData + ReadonlyAccountOwner>(
    sanctum_spl_stake_pool: S,
) -> Result<SplStakePool, GenericPoolCalculatorError> {
    deserialize_stake_pool_check_program_owner(
        sanctum_spl_stake_pool,
        SanctumSplSolValCalc::POOL_PROGRAM_ID,
    )
}

pub fn deserialize_sanctum_spl_multi_stake_pool_checked<
    S: ReadonlyAccountData + ReadonlyAccountOwner,
>(
    sanctum_spl_stake_pool: S,
) -> Result<SplStakePool, GenericPoolCalculatorError> {
    deserialize_stake_pool_check_program_owner(
        sanctum_spl_stake_pool,
        SanctumSplMultiSolValCalc::POOL_PROGRAM_ID,
    )
}

fn deserialize_stake_pool_check_program_owner<S: ReadonlyAccountData + ReadonlyAccountOwner>(
    spl_stake_pool: S,
    stake_pool_program_id: Pubkey,
) -> Result<SplStakePool, GenericPoolCalculatorError> {
    if *spl_stake_pool.owner() != stake_pool_program_id {
        return Err(GenericPoolCalculatorError::InvalidStakePoolProgramData);
    }
    deserialize_stake_pool_checked(spl_stake_pool)
}

/// Deserializes a stake pool from account data,
/// checking AccountType (if its an initialized stake pool),
/// but not owner (if its owned by the correct stake pool program)
pub fn deserialize_stake_pool_checked<D: ReadonlyAccountData>(
    spl_stake_pool: D,
) -> Result<SplStakePool, GenericPoolCalculatorError> {
    let stake_pool = SplStakePool::deserialize(&mut spl_stake_pool.data().as_ref())
        .map_err(|_e| GenericPoolCalculatorError::InvalidStakePoolProgramData)?;
    if stake_pool.account_type != AccountType::StakePool {
        return Err(GenericPoolCalculatorError::InvalidStakePoolProgramData);
    }
    Ok(stake_pool)
}

#[derive(Clone, Copy, Debug)]
pub struct SplLstSolCommonFreeArgs<S, Q> {
    pub spl_stake_pool: S,
    pub spl_stake_pool_prog: Q,
}

impl<
        S: ReadonlyAccountPubkey + ReadonlyAccountData + ReadonlyAccountOwner,
        Q: ReadonlyAccountPubkey + ReadonlyAccountData,
    > SplLstSolCommonFreeArgs<S, Q>
{
    pub fn resolve_spl(
        self,
    ) -> Result<(LstSolCommonIntermediateArgs<Q>, SplStakePool), GenericPoolCalculatorError> {
        self.resolve_for_stake_pool_program(SplSolValCalc::POOL_PROGRAM_ID)
    }

    pub fn resolve_sanctum_spl(
        self,
    ) -> Result<(LstSolCommonIntermediateArgs<Q>, SplStakePool), GenericPoolCalculatorError> {
        self.resolve_for_stake_pool_program(SanctumSplSolValCalc::POOL_PROGRAM_ID)
    }

    pub fn resolve_sanctum_spl_multi(
        self,
    ) -> Result<(LstSolCommonIntermediateArgs<Q>, SplStakePool), GenericPoolCalculatorError> {
        self.resolve_for_stake_pool_program(SanctumSplMultiSolValCalc::POOL_PROGRAM_ID)
    }

    fn resolve_for_stake_pool_program(
        self,
        stake_pool_program_id: Pubkey,
    ) -> Result<(LstSolCommonIntermediateArgs<Q>, SplStakePool), GenericPoolCalculatorError> {
        if *self.spl_stake_pool_prog.pubkey() != stake_pool_program_id {
            return Err(GenericPoolCalculatorError::WrongPoolProgram);
        }
        let stake_pool = deserialize_stake_pool_check_program_owner(
            &self.spl_stake_pool,
            stake_pool_program_id,
        )?;
        Ok((
            LstSolCommonIntermediateArgs {
                lst_mint: stake_pool.pool_mint,
                pool_state: *self.spl_stake_pool.pubkey(),
                pool_program: self.spl_stake_pool_prog,
            },
            stake_pool,
        ))
    }
}

/// Struct that uses defined const for POOL_PROGRAM_PROGDATA
/// so that it can be used on client side without fetching POOL_PROGRAM
#[derive(Clone, Copy, Debug)]
pub struct SplLstSolCommonFreeArgsConst<S> {
    pub spl_stake_pool: S,
}

impl<S: ReadonlyAccountPubkey + ReadonlyAccountData + ReadonlyAccountOwner>
    SplLstSolCommonFreeArgsConst<S>
{
    pub fn resolve_spl(self) -> Result<LstSolCommonIntermediateKeys, GenericPoolCalculatorError> {
        let stake_pool = deserialize_spl_stake_pool_checked(&self.spl_stake_pool)?;
        Ok(LstSolCommonIntermediateKeys {
            lst_mint: stake_pool.pool_mint,
            pool_state: *self.spl_stake_pool.pubkey(),
        })
    }

    pub fn resolve_spl_to_account_metas(
        self,
    ) -> Result<[AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN], GenericPoolCalculatorError> {
        let keys = self.resolve_spl()?;
        Ok(resolve_to_account_metas_for_calc::<SplSolValCalc>(keys))
    }

    pub fn resolve_sanctum_spl(
        self,
    ) -> Result<LstSolCommonIntermediateKeys, GenericPoolCalculatorError> {
        let stake_pool = deserialize_sanctum_spl_stake_pool_checked(&self.spl_stake_pool)?;
        Ok(LstSolCommonIntermediateKeys {
            lst_mint: stake_pool.pool_mint,
            pool_state: *self.spl_stake_pool.pubkey(),
        })
    }

    pub fn resolve_sanctum_spl_to_account_metas(
        self,
    ) -> Result<[AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN], GenericPoolCalculatorError> {
        let keys = self.resolve_sanctum_spl()?;
        Ok(resolve_to_account_metas_for_calc::<SanctumSplSolValCalc>(
            keys,
        ))
    }

    pub fn resolve_sanctum_spl_multi(
        self,
    ) -> Result<LstSolCommonIntermediateKeys, GenericPoolCalculatorError> {
        let stake_pool = deserialize_sanctum_spl_multi_stake_pool_checked(&self.spl_stake_pool)?;
        Ok(LstSolCommonIntermediateKeys {
            lst_mint: stake_pool.pool_mint,
            pool_state: *self.spl_stake_pool.pubkey(),
        })
    }

    pub fn resolve_sanctum_spl_multi_to_account_metas(
        self,
    ) -> Result<[AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN], GenericPoolCalculatorError> {
        let keys = self.resolve_sanctum_spl_multi()?;
        Ok(resolve_to_account_metas_for_calc::<SanctumSplMultiSolValCalc>(keys))
    }
}

pub fn resolve_to_account_metas_for_calc<T: GenericPoolSolValCalc>(
    keys: LstSolCommonIntermediateKeys,
) -> [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] {
    let keys: generic_pool_calculator_interface::LstToSolKeys = keys.resolve::<T>().into();
    keys.into()
}
