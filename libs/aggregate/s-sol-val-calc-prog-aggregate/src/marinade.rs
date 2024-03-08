use borsh::BorshDeserialize;
use marinade_calculator_interface::MarinadeState;
use marinade_calculator_lib::{
    MarinadeSolValCalc, MarinadeStateCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};
use marinade_keys::{marinade_state, msol};
use sanctum_token_ratio::U64ValueRange;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use spl_calculator_lib::resolve_to_account_metas_for_calc;
use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{KnownLstSolValCalc, LstSolValCalc, LstSolValCalcErr, MutableLstSolValCalc};

#[derive(Clone, Copy, Debug, Default)]
pub struct MarinadeLstSolValCalc {
    pub calc: Option<MarinadeStateCalc>,
}

impl MutableLstSolValCalc for MarinadeLstSolValCalc {
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![marinade_state::ID]
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(acc) = account_map.get(&marinade_state::ID) {
            self.calc = Some(MarinadeStateCalc::from(MarinadeState::deserialize(
                &mut acc.data().as_ref(),
            )?));
        }
        Ok(())
    }
}

impl LstSolValCalc for MarinadeLstSolValCalc {
    fn lst_mint(&self) -> Pubkey {
        msol::ID
    }

    fn lst_to_sol(&self, lst_amount: u64) -> Result<U64ValueRange, Box<dyn Error + Send + Sync>> {
        let calc = self.calc.ok_or(MarinadeLstSolValCalcErr::StateNotFetched)?;
        calc.verify_can_withdraw_stake()?;
        Ok(calc.calc_lst_to_sol(lst_amount)?)
    }

    fn sol_to_lst(&self, lamports: u64) -> Result<U64ValueRange, Box<dyn Error + Send + Sync>> {
        let calc = self.calc.ok_or(MarinadeLstSolValCalcErr::StateNotFetched)?;
        calc.verify_can_withdraw_stake()?;
        Ok(calc.calc_sol_to_lst(lamports)?)
    }

    fn ix_accounts(&self) -> Vec<AccountMeta> {
        Vec::from(resolve_to_account_metas_for_calc::<MarinadeSolValCalc>(
            MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MarinadeLstSolValCalcErr {
    StateNotFetched,
}

impl Display for MarinadeLstSolValCalcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateNotFetched => f.write_str("marinade state not yet fetched"),
        }
    }
}

impl Error for MarinadeLstSolValCalcErr {}

impl TryFrom<KnownLstSolValCalc> for MarinadeLstSolValCalc {
    type Error = LstSolValCalcErr;

    fn try_from(value: KnownLstSolValCalc) -> Result<Self, Self::Error> {
        match value {
            KnownLstSolValCalc::Marinade(s) => Ok(s),
            _ => Err(LstSolValCalcErr::WrongLstSolValCalc),
        }
    }
}
