use anyhow::anyhow;
use jupiter_amm_interface::{
    AccountMap, Amm, KeyedAccount, Quote, QuoteParams, SwapAndAccountMetas, SwapMode, SwapParams,
};
use s_controller_interface::LstState;
use s_controller_lib::find_lst_state_list_address;
use s_pricing_prog_aggregate::MutablePricingProg;
use s_sol_val_calc_prog_aggregate::MutableLstSolValCalc;
use sanctum_lst_list::SanctumLstList;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::SPoolJup;

pub const LABEL: &str = "Sanctum Infinity";

impl Amm for SPoolJup {
    /// Initialized by lst_state_list account, NOT pool_state.
    ///
    /// params can optionally be a b58-encoded pubkey string that is the S controller program's program_id.
    ///
    /// Must be updated 2 more times before it can be used, see docs for [`Self::from_lst_state_list_account`]
    fn from_keyed_account(
        KeyedAccount {
            key,
            account,
            params,
        }: &KeyedAccount,
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
        Self::from_lst_state_list_account(program_id, account.clone(), &sanctum_lst_list)
    }

    fn label(&self) -> String {
        LABEL.into()
    }

    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    /// S Pools are 1 per program, so just use program ID as key
    fn key(&self) -> Pubkey {
        self.program_id()
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        let mut res: Vec<Pubkey> = match self.lst_state_list() {
            Ok(list) => list.iter().map(|LstState { mint, .. }| *mint).collect(),
            Err(_e) => vec![],
        };
        if let Ok(pool_state) = self.pool_state() {
            res.push(pool_state.lp_token_mint);
        }
        res
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        let mut res = vec![self.lst_state_list_addr, self.pool_state_addr];
        if let Some(pricing_prog) = &self.pricing_prog {
            res.extend(pricing_prog.get_accounts_to_update());
        }
        if let Ok(pool_state) = &self.pool_state() {
            res.push(pool_state.lp_token_mint);
        }
        if let Ok(lst_state_list) = self.lst_state_list() {
            res.extend(
                lst_state_list
                    .iter()
                    .zip(self.lst_data_list.iter())
                    .filter_map(|(lst_state, lst_data)| {
                        let lst_data = lst_data.as_ref()?;
                        let mut res = lst_data.sol_val_calc.get_accounts_to_update();
                        if let Ok(ata) = self.pool_reserves_account(lst_state, lst_data) {
                            res.push(ata);
                        }
                        Some(res)
                    })
                    .flatten(),
            );
        }
        res
    }

    fn update(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        // returns the first encountered error, but tries to update everything eagerly
        // even after encountering an error

        // first update lst_data_list and pricing_program
        //
        // update pool_state and lst_state_list afterwards so we can invalidate
        // pricing_prog and lst_sol_val_calcs if any of them changed
        //  - update lst_state_list before pool_state so we can use the new lst_state_list to reinitialize pricing program if required
        //
        // finally update LP token supply using the newest pool state
        self.update_lst_data_list(account_map)
            .and(self.update_pricing_prog(account_map))
            .and(self.update_lst_state_list(account_map))
            .and(self.update_pool_state(account_map))
            .and(self.update_lp_token_supply(account_map))
    }

    fn quote(&self, quote_params: &QuoteParams) -> anyhow::Result<Quote> {
        let lp_mint = self.pool_state()?.lp_token_mint;
        if quote_params.input_mint == lp_mint {
            unimplemented!("remove liquidity");
        } else if quote_params.output_mint == lp_mint {
            unimplemented!("add liquidity")
        } else {
            match quote_params.swap_mode {
                SwapMode::ExactIn => self.quote_swap_exact_in(quote_params),
                SwapMode::ExactOut => unimplemented!("swap exact out"),
            }
        }
    }

    fn get_swap_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        let lp_mint = self.pool_state()?.lp_token_mint;
        if swap_params.source_mint == lp_mint {
            unimplemented!("remove liquidity");
        } else if swap_params.destination_mint == lp_mint {
            unimplemented!("add liquidity")
        } else {
            // TODO: wtf where did swap_params.swap_mode go?
            // right now if output == 0 => assume ExactIn
            if swap_params.out_amount == 0 {
                self.swap_exact_in_swap_and_account_metas(swap_params)
            } else {
                unimplemented!("swap exact out")
            }
        }
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
}
