use generic_pool_calculator_interface::GenericPoolCalculatorError;
use generic_pool_calculator_lib::account_resolvers::LstSolCommonIntermediateKeys;
use sanctum_token_ratio::U64ValueRange;
use sol_value_calculator_lib::SolValueCalculator;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountOwner, ReadonlyAccountPubkey};
use spl_calculator_lib::{
    deserialize_sanctum_spl_multi_stake_pool_checked, resolve_to_account_metas_for_calc,
    sanctum_spl_multi_sol_val_calc_program, SanctumSplMultiSolValCalc, SplStakePoolCalc,
};
use std::collections::HashMap;

use crate::{
    KnownLstSolValCalc, LstSolValCalc, LstSolValCalcErr, MutableLstSolValCalc, SplLstSolValCalc,
    SplLstSolValCalcInitKeys,
};

#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct SanctumSplMultiLstSolValCalc(pub SplLstSolValCalc);

impl SanctumSplMultiLstSolValCalc {
    pub fn from_keys(keys: SplLstSolValCalcInitKeys) -> Self {
        Self(SplLstSolValCalc::from_keys(keys))
    }

    pub fn from_pool<P: ReadonlyAccountData + ReadonlyAccountPubkey + ReadonlyAccountOwner>(
        pool_acc: P,
    ) -> Result<Self, GenericPoolCalculatorError> {
        let stake_pool_addr = *pool_acc.pubkey();
        let pool = deserialize_sanctum_spl_multi_stake_pool_checked(pool_acc)?;
        Ok(Self(SplLstSolValCalc {
            lst_mint: pool.pool_mint,
            stake_pool_addr,
            calc: Some(SplStakePoolCalc::from(pool)),
            clock: None,
        }))
    }
}

impl MutableLstSolValCalc for SanctumSplMultiLstSolValCalc {
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![sysvar::clock::ID, self.0.stake_pool_addr]
    }

    fn update<D: ReadonlyAccountData>(
        &mut self,
        account_map: &HashMap<Pubkey, D>,
    ) -> anyhow::Result<()> {
        self.0.update(account_map)
    }
}

impl LstSolValCalc for SanctumSplMultiLstSolValCalc {
    fn sol_value_calculator_program_id(&self) -> Pubkey {
        sanctum_spl_multi_sol_val_calc_program::ID
    }

    fn ix_accounts(&self) -> Vec<AccountMeta> {
        Vec::from(
            resolve_to_account_metas_for_calc::<SanctumSplMultiSolValCalc>(
                LstSolCommonIntermediateKeys {
                    lst_mint: self.0.lst_mint,
                    pool_state: self.0.stake_pool_addr,
                },
            ),
        )
    }

    fn lst_mint(&self) -> Pubkey {
        self.0.lst_mint()
    }

    fn lst_to_sol(&self, lst_amount: u64) -> anyhow::Result<U64ValueRange> {
        self.0.lst_to_sol(lst_amount)
    }

    fn sol_to_lst(&self, lamports: u64) -> anyhow::Result<U64ValueRange> {
        self.0.sol_to_lst(lamports)
    }

    fn sol_value_calculator(&self) -> Option<&dyn SolValueCalculator> {
        self.0.sol_value_calculator()
    }
}

impl TryFrom<KnownLstSolValCalc> for SanctumSplMultiLstSolValCalc {
    type Error = LstSolValCalcErr;

    fn try_from(value: KnownLstSolValCalc) -> Result<Self, Self::Error> {
        match value {
            KnownLstSolValCalc::SanctumSplMulti(s) => Ok(s),
            _ => Err(LstSolValCalcErr::WrongLstSolValCalc),
        }
    }
}
