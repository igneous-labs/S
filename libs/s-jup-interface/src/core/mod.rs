//! Core jup quoting and swapping functionality

use anyhow::anyhow;
use jupiter_amm_interface::{SwapMode, SwapParams};
use solana_sdk::instruction::Instruction;

use crate::SPoolJup;

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

impl SPoolJup {
    // Allows for use with transactions without jup program
    pub fn swap_ix(
        &self,
        swap_params: &SwapParams,
        swap_mode: SwapMode, // to make up for lack of swap_mode in swap_params
    ) -> anyhow::Result<Instruction> {
        let lp_mint = self.pool_state()?.lp_token_mint;
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
}
