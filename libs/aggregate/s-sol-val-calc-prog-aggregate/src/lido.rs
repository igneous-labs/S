use borsh::BorshDeserialize;
use lido_calculator_interface::Lido;
use lido_calculator_lib::{LidoCalc, LidoSolValCalc, LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS};
use lido_keys::{lido_state, stsol};
use sanctum_token_ratio::U64ValueRange;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{clock::Clock, instruction::AccountMeta, pubkey::Pubkey, sysvar};
use solana_readonly_account::ReadonlyAccountData;
use spl_calculator_lib::resolve_to_account_metas_for_calc;
use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{KnownLstSolValCalc, LstSolValCalc, LstSolValCalcErr, MutableLstSolValCalc};

#[derive(Clone, Debug, Default)]
pub struct LidoLstSolValCalc {
    pub calc: Option<LidoCalc>,
    pub clock: Option<Clock>,
}

impl MutableLstSolValCalc for LidoLstSolValCalc {
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![lido_state::ID, sysvar::clock::ID]
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(acc) = account_map.get(&sysvar::clock::ID) {
            self.clock = Some(bincode::deserialize::<Clock>(&acc.data())?);
        }
        if let Some(acc) = account_map.get(&lido_state::ID) {
            self.calc = Some(LidoCalc::from(Lido::deserialize(&mut acc.data().as_ref())?));
        }
        Ok(())
    }
}

impl LstSolValCalc for LidoLstSolValCalc {
    fn sol_value_calculator_program_id(&self) -> Pubkey {
        lido_calculator_lib::program::ID
    }

    fn lst_mint(&self) -> Pubkey {
        stsol::ID
    }

    fn lst_to_sol(&self, lst_amount: u64) -> Result<U64ValueRange, Box<dyn Error + Send + Sync>> {
        let calc = self.calc.ok_or(LidoLstSolValCalcErr::StateNotFetched)?;
        let clock = self
            .clock
            .as_ref()
            .ok_or(LidoLstSolValCalcErr::ClockNotFetched)?;
        calc.verify_pool_updated_for_this_epoch(clock)?;
        Ok(calc.calc_lst_to_sol(lst_amount)?)
    }

    fn sol_to_lst(&self, lamports: u64) -> Result<U64ValueRange, Box<dyn Error + Send + Sync>> {
        let calc = self.calc.ok_or(LidoLstSolValCalcErr::StateNotFetched)?;
        let clock = self
            .clock
            .as_ref()
            .ok_or(LidoLstSolValCalcErr::ClockNotFetched)?;
        calc.verify_pool_updated_for_this_epoch(clock)?;
        Ok(calc.calc_sol_to_lst(lamports)?)
    }

    fn ix_accounts(&self) -> Vec<AccountMeta> {
        Vec::from(resolve_to_account_metas_for_calc::<LidoSolValCalc>(
            LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LidoLstSolValCalcErr {
    StateNotFetched,
    ClockNotFetched,
}

impl Display for LidoLstSolValCalcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateNotFetched => f.write_str("lido state not yet fetched"),
            Self::ClockNotFetched => f.write_str("clock not yet fetched"),
        }
    }
}

impl Error for LidoLstSolValCalcErr {}

impl TryFrom<KnownLstSolValCalc> for LidoLstSolValCalc {
    type Error = LstSolValCalcErr;

    fn try_from(value: KnownLstSolValCalc) -> Result<Self, Self::Error> {
        match value {
            KnownLstSolValCalc::Lido(s) => Ok(s),
            _ => Err(LstSolValCalcErr::WrongLstSolValCalc),
        }
    }
}
