use anyhow::anyhow;
use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, SwapAndAccountMetas, SwapParams,
};
use s_controller_lib::find_lst_state_list_address;
use sanctum_lst_list::{
    lido_program, marinade_program, sanctum_spl_multi_stake_pool_program,
    sanctum_spl_stake_pool_program, spl_stake_pool_program, SanctumLstList,
};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::SPoolJup;

pub const LABEL: &str = "Sanctum Infinity";

impl Amm for SPoolJup {
    /// Initialized by lst_state_list account, NOT pool_state.
    ///
    /// params can optionally be a b58-encoded pubkey string that is the S controller program's program_id.
    ///
    /// Must be updated 2 more times before it can be used, see docs for [`Self::from_lst_state_list_account`].
    ///
    /// TODO: We can also repurpose params to pass in a [`SanctumLstList`] in order to allow dynamic reloading of list
    fn from_keyed_account(
        KeyedAccount {
            key,
            account,
            params,
        }: &KeyedAccount,
        _amm_context: &AmmContext,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let (program_id, lst_state_list_addr) = match params {
            // default to INF if program_id params not provided
            None => (
                s_controller_lib::program::ID,
                s_controller_lib::program::LST_STATE_LIST_ID,
            ),
            Some(value) => {
                // TODO: maybe unnecessary clone() here?
                let program_id =
                    Pubkey::from_str(&serde_json::from_value::<String>(value.clone())?)?;
                (program_id, find_lst_state_list_address(program_id).0)
            }
        };
        if *key != lst_state_list_addr {
            return Err(anyhow!(
                "Incorrect LST state list addr. Expected {lst_state_list_addr}. Got {key}"
            ));
        }
        let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();
        Self::from_lst_state_list_account_and_sanctum_lst_list(
            program_id,
            account.clone(),
            &sanctum_lst_list,
        )
    }

    fn label(&self) -> String {
        LABEL.into()
    }

    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    fn key(&self) -> Pubkey {
        self.lst_state_list_addr
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        self.get_reserve_mints_full()
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.get_accounts_to_update_full()
    }

    fn update(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        self.update_full(account_map)
    }

    fn quote(&self, quote_params: &QuoteParams) -> anyhow::Result<Quote> {
        self.quote_full(quote_params)
    }

    fn get_swap_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        let mut swap_and_account_metas = self.get_swap_and_account_metas_full(swap_params)?;

        if !swap_params.token_transfer_authority.is_on_curve() {
            for account_meta in swap_and_account_metas.account_metas.iter_mut() {
                if account_meta.is_signer {
                    account_meta.is_signer = false;
                }
            }
        }
        swap_and_account_metas
            .account_metas
            .push(swap_params.placeholder_account_meta());
        Ok(swap_and_account_metas)
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn has_dynamic_accounts(&self) -> bool {
        true
    }

    /// TODO: this is not true for AddLiquidity and RemoveLiquidity
    fn supports_exact_out(&self) -> bool {
        true
    }

    fn requires_update_for_reserve_mints(&self) -> bool {
        true
    }

    fn program_dependencies(&self) -> Vec<(Pubkey, String)> {
        PROGRAM_DEPENDENCIES
            .into_iter()
            .map(|(address, label)| (address, label.into()))
            .collect()
    }
}

// stake pool programs are commented out for now
// since we dont execute them and we haven't done
// account subslicing for program data account data yet
pub const PROGRAM_DEPENDENCIES: [(Pubkey, &str); 12] = [
    // SPL
    (spl_stake_pool_program::ID, "spl_stake_pool"),
    (spl_calculator_lib::program::ID, "spl_calculator"),
    // Sanctum SPL
    (sanctum_spl_stake_pool_program::ID, "sanctum_spl_stake_pool"),
    (
        spl_calculator_lib::sanctum_spl_sol_val_calc_program::ID,
        "sanctum_spl_calculator",
    ),
    // Sanctum SPL Multi
    (
        sanctum_spl_multi_stake_pool_program::ID,
        "sanctum_spl_multi_stake_pool",
    ),
    (
        spl_calculator_lib::sanctum_spl_multi_sol_val_calc_program::ID,
        "sanctum_spl_multi_calculator",
    ),
    // marinade
    (marinade_program::ID, "marinade"),
    (marinade_calculator_lib::program::ID, "marinade_calculator"),
    // lido
    (lido_program::ID, "lido"),
    (lido_calculator_lib::program::ID, "lido_calculator"),
    // wSOL
    (wsol_calculator_lib::program::ID, "wsol_calculator"),
    // pricing program
    (flat_fee_interface::ID, "flat_fee_pricing_program"),
];
