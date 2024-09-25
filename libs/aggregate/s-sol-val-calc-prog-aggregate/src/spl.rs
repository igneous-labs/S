use generic_pool_calculator_interface::GenericPoolCalculatorError;
use generic_pool_calculator_lib::account_resolvers::LstSolCommonIntermediateKeys;
use sanctum_token_ratio::U64ValueRange;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};
use spl_calculator_lib::{
    deserialize_spl_stake_pool_checked, deserialize_stake_pool_checked,
    resolve_to_account_metas_for_calc, SplSolValCalc, SplStakePoolCalc,
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use crate::{KnownLstSolValCalc, LstSolValCalc, LstSolValCalcErr, MutableLstSolValCalc};

#[derive(Clone, Debug, Default)]
pub struct SplLstSolValCalc {
    pub lst_mint: Pubkey,
    pub stake_pool_addr: Pubkey,
    pub calc: Option<SplStakePoolCalc>,
    pub shared_current_epoch: Arc<AtomicU64>,
}

#[derive(Clone, Copy, Debug)]
pub struct SplLstSolValCalcInitKeys {
    pub lst_mint: Pubkey,
    pub stake_pool_addr: Pubkey,
}

impl SplLstSolValCalc {
    #[inline]
    pub const fn from_keys(
        SplLstSolValCalcInitKeys {
            lst_mint,
            stake_pool_addr,
        }: SplLstSolValCalcInitKeys,
        shared_current_epoch: Arc<AtomicU64>,
    ) -> Self {
        Self {
            lst_mint,
            stake_pool_addr,
            calc: None,
            shared_current_epoch,
        }
    }

    #[inline]
    pub fn from_pool<P: ReadonlyAccountData + ReadonlyAccountPubkey + ReadonlyAccountOwner>(
        pool_acc: P,
        shared_current_epoch: Arc<AtomicU64>,
    ) -> Result<Self, GenericPoolCalculatorError> {
        let stake_pool_addr = *pool_acc.pubkey();
        let pool = deserialize_spl_stake_pool_checked(pool_acc)?;
        Ok(Self {
            lst_mint: pool.pool_mint,
            stake_pool_addr,
            calc: Some(SplStakePoolCalc::from(pool)),
            shared_current_epoch,
        })
    }

    #[inline]
    pub fn current_epoch(&self) -> u64 {
        self.shared_current_epoch.load(Ordering::Relaxed)
    }
}

impl MutableLstSolValCalc for SplLstSolValCalc {
    #[inline]
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![self.stake_pool_addr]
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> anyhow::Result<()> {
        if let Some(acc) = account_map.get(&self.stake_pool_addr) {
            let pool = deserialize_stake_pool_checked(acc)?;
            if pool.pool_mint != self.lst_mint {
                return Err(SplLstSolValCalcErr::WrongLstMint.into());
            }
            self.calc = Some(SplStakePoolCalc::from(pool));
        }
        Ok(())
    }
}

impl LstSolValCalc for SplLstSolValCalc {
    #[inline]
    fn sol_value_calculator_program_id(&self) -> Pubkey {
        spl_calculator_lib::program::ID
    }

    #[inline]
    fn lst_mint(&self) -> Pubkey {
        self.lst_mint
    }

    fn lst_to_sol(&self, lst_amount: u64) -> anyhow::Result<U64ValueRange> {
        let calc = self.calc.ok_or(SplLstSolValCalcErr::StakePoolNotFetched)?;
        calc.verify_pool_updated_for_this_epoch(self.current_epoch())?;
        Ok(calc.calc_lst_to_sol(lst_amount)?)
    }

    fn sol_to_lst(&self, lamports: u64) -> anyhow::Result<U64ValueRange> {
        let calc = self.calc.ok_or(SplLstSolValCalcErr::StakePoolNotFetched)?;
        calc.verify_pool_updated_for_this_epoch(self.current_epoch())?;
        Ok(calc.calc_sol_to_lst(lamports)?)
    }

    fn ix_accounts(&self) -> Vec<AccountMeta> {
        Vec::from(resolve_to_account_metas_for_calc::<SplSolValCalc>(
            LstSolCommonIntermediateKeys {
                lst_mint: self.lst_mint,
                pool_state: self.stake_pool_addr,
            },
        ))
    }

    #[inline]
    fn sol_value_calculator(&self) -> Option<&dyn SolValueCalculator> {
        self.calc.as_ref().map(|c| c as &dyn SolValueCalculator)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SplLstSolValCalcErr {
    WrongLstMint,
    StakePoolNotFetched,
}

impl Display for SplLstSolValCalcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongLstMint => f.write_str("LST mint and stake pool does not match"),
            Self::StakePoolNotFetched => f.write_str("stake pool not yet fetched"),
        }
    }
}

impl Error for SplLstSolValCalcErr {}

impl TryFrom<KnownLstSolValCalc> for SplLstSolValCalc {
    type Error = LstSolValCalcErr;

    fn try_from(value: KnownLstSolValCalc) -> Result<Self, Self::Error> {
        match value {
            KnownLstSolValCalc::Spl(s) => Ok(s),
            _ => Err(LstSolValCalcErr::WrongLstSolValCalc),
        }
    }
}
