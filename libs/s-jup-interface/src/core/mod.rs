//! Core jup quoting and swapping functionality

use anyhow::anyhow;
use jupiter_amm_interface::{Quote, QuoteParams, SwapAndAccountMetas, SwapMode, SwapParams};
use s_controller_interface::LstState;
use s_controller_lib::try_lst_state_list;
use solana_readonly_account::ReadonlyAccountData;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use crate::SPool;

mod add_liquidity;
mod common;
mod remove_liquidity;
mod swap_exact_in;
mod swap_exact_out;

pub use add_liquidity::*;
pub use remove_liquidity::*;
pub use swap_exact_in::*;
pub use swap_exact_out::*;

use common::*;

impl<S: ReadonlyAccountData, L: ReadonlyAccountData> SPool<S, L> {
    // Allows for use with transactions without jup program
    pub fn swap_ix(
        &self,
        swap_params: &SwapParams,
        swap_mode: SwapMode, // to make up for lack of swap_mode in swap_params
    ) -> anyhow::Result<Instruction> {
        let lp_mint = self.lp_token_mint()?;
        if swap_params.source_mint == lp_mint {
            if let SwapMode::ExactOut = swap_mode {
                return Err(anyhow!("ExactOut not supported for remove liquidity"));
            }
            self.remove_liquidity_ix(swap_params)
        } else if swap_params.destination_mint == lp_mint {
            if let SwapMode::ExactOut = swap_mode {
                return Err(anyhow!("ExactOut not supported for add liquidity"));
            }
            self.add_liquidity_ix(swap_params)
        } else {
            match swap_mode {
                SwapMode::ExactIn => self.swap_exact_in_ix(swap_params),
                SwapMode::ExactOut => self.swap_exact_out_ix(swap_params),
            }
        }
    }

    pub fn quote_full(&self, quote_params: &QuoteParams) -> anyhow::Result<Quote> {
        let lp_mint = self.lp_token_mint()?;
        if quote_params.input_mint == lp_mint {
            if let SwapMode::ExactOut = quote_params.swap_mode {
                return Err(anyhow!("ExactOut not supported for remove liquidity"));
            }
            self.quote_remove_liquidity(quote_params)
        } else if quote_params.output_mint == lp_mint {
            if let SwapMode::ExactOut = quote_params.swap_mode {
                return Err(anyhow!("ExactOut not supported for add liquidity"));
            }
            self.quote_add_liquidity(quote_params)
        } else {
            match quote_params.swap_mode {
                SwapMode::ExactIn => self.quote_swap_exact_in(quote_params),
                SwapMode::ExactOut => self.quote_swap_exact_out(quote_params),
            }
        }
    }

    pub fn get_swap_and_account_metas_full(
        &self,
        swap_params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        let lp_mint = self.lp_token_mint()?;
        if swap_params.source_mint == lp_mint {
            self.remove_liquidity_swap_and_account_metas(swap_params)
        } else if swap_params.destination_mint == lp_mint {
            self.add_liquidity_swap_and_account_metas(swap_params)
        } else {
            // TODO: wtf where did swap_params.swap_mode go?
            // right now if in_amount == 0 => assume ExactOut
            if swap_params.in_amount == 0 {
                self.swap_exact_out_swap_and_account_metas(swap_params)
            } else {
                self.swap_exact_in_swap_and_account_metas(swap_params)
            }
        }
    }

    /// Returns all mints this SPool can swap between (includes LP token mint)
    pub fn get_reserve_mints_full(&self) -> Vec<Pubkey> {
        let lst_state_list_data = self.lst_state_list_account.data();
        let mut res: Vec<Pubkey> = try_lst_state_list(&lst_state_list_data).map_or_else(
            |_e| vec![],
            |list| list.iter().map(|LstState { mint, .. }| *mint).collect(),
        );
        if let Ok(lp_token_mint) = self.lp_token_mint() {
            res.push(lp_token_mint);
        }
        res
    }
}
