//! Core jup quoting and swapping functionality

use jupiter_amm_interface::SwapParams;
use solana_sdk::instruction::Instruction;

use crate::SPoolJup;

mod common;
mod swap_exact_in;
mod swap_exact_out;

pub use swap_exact_in::*;
pub use swap_exact_out::*;

use common::*;

impl SPoolJup {
    // Used for testing before jup has SwapAndAccountMetas updated
    pub fn swap_ix(&self, swap_params: &SwapParams) -> anyhow::Result<Instruction> {
        let lp_mint = self.pool_state()?.lp_token_mint;
        if swap_params.source_mint == lp_mint {
            unimplemented!("remove liquidity");
        } else if swap_params.destination_mint == lp_mint {
            unimplemented!("add liquidity")
        } else {
            // TODO: wtf where did swap_params.swap_mode go?
            // right now if output == 0 => assume ExactIn
            if swap_params.out_amount == 0 {
                self.swap_exact_in_ix(swap_params)
            } else {
                self.swap_exact_out_ix(swap_params)
            }
        }
    }
}
