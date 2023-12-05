use s_controller_interface::InitializeKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::{initial_authority, program::POOL_STATE_ID};

#[derive(Clone, Copy, Debug)]
pub struct InitializeFreeArgs {
    pub payer: Pubkey,
    pub lp_token_mint: Pubkey,
}

impl InitializeFreeArgs {
    pub fn resolve(self) -> InitializeKeys {
        let InitializeFreeArgs {
            payer,
            lp_token_mint,
        } = self;
        InitializeKeys {
            payer,
            lp_token_mint,
            authority: initial_authority::ID,
            pool_state: POOL_STATE_ID,
            token_2022: spl_token_2022::ID,
            system_program: system_program::ID,
        }
    }
}
