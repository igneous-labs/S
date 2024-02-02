use s_controller_interface::InitializeKeys;
use solana_program::{pubkey::Pubkey, system_program};

use crate::{find_pool_state_address, initial_authority, program::POOL_STATE_ID};

#[derive(Clone, Copy, Debug)]
pub struct InitializeFreeArgs {
    pub payer: Pubkey,
    pub lp_token_mint: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct InitializeResolveForProg {
    pub program_id: Pubkey,
    pub initial_authority: Pubkey,
}

impl InitializeFreeArgs {
    pub fn resolve(self) -> InitializeKeys {
        let Self {
            payer,
            lp_token_mint,
        } = self;
        InitializeKeys {
            payer,
            lp_token_mint,
            authority: initial_authority::ID,
            pool_state: POOL_STATE_ID,
            lp_token_program: spl_token::ID,
            system_program: system_program::ID,
        }
    }

    pub fn resolve_for_prog(
        self,
        InitializeResolveForProg {
            program_id,
            initial_authority,
        }: InitializeResolveForProg,
    ) -> InitializeKeys {
        let Self {
            payer,
            lp_token_mint,
        } = self;
        InitializeKeys {
            payer,
            authority: initial_authority,
            pool_state: find_pool_state_address(program_id).0,
            lp_token_mint,
            lp_token_program: spl_token::ID,
            system_program: system_program::ID,
        }
    }
}
