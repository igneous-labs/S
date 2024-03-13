use sanctum_token_ratio::U64ValueRange;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use std::collections::HashMap;
use wsol_calculator_lib::{WsolSolCalc, WSOL_LST_SOL_COMMON_METAS};
use wsol_keys::wsol;

use crate::{KnownLstSolValCalc, LstSolValCalc, LstSolValCalcErr, MutableLstSolValCalc};

#[derive(Clone, Copy, Debug, Default)]
pub struct WsolLstSolValCalc;

impl MutableLstSolValCalc for WsolLstSolValCalc {
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        Vec::new()
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        _account_map: &HashMap<Pubkey, D>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

impl LstSolValCalc for WsolLstSolValCalc {
    fn sol_value_calculator_program_id(&self) -> Pubkey {
        wsol_calculator_lib::program::ID
    }

    fn lst_mint(&self) -> Pubkey {
        wsol::ID
    }

    fn lst_to_sol(&self, lst_amount: u64) -> anyhow::Result<U64ValueRange> {
        Ok(WsolSolCalc.calc_lst_to_sol(lst_amount)?)
    }

    fn sol_to_lst(&self, lamports: u64) -> anyhow::Result<U64ValueRange> {
        Ok(WsolSolCalc.calc_sol_to_lst(lamports)?)
    }

    fn ix_accounts(&self) -> Vec<AccountMeta> {
        WSOL_LST_SOL_COMMON_METAS.into()
    }
}

impl TryFrom<KnownLstSolValCalc> for WsolLstSolValCalc {
    type Error = LstSolValCalcErr;

    fn try_from(value: KnownLstSolValCalc) -> Result<Self, Self::Error> {
        match value {
            KnownLstSolValCalc::Wsol(s) => Ok(s),
            _ => Err(LstSolValCalcErr::WrongLstSolValCalc),
        }
    }
}
