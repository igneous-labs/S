use generic_pool_calculator_lib::GenericPoolSolValCalc;
use lido_keys::{lido_program, lido_program_progdata};
use solana_program::pubkey::Pubkey;

mod calc;
mod instructions;

pub use account_resolvers::*;
pub use calc::*;
pub use instructions::*;

pub mod program {
    sanctum_macros::declare_program_keys!(
        "1idUSy4MGGKyKhvjSnGZ6Zc7Q4eKQcibym4BkEEw9KR",
        [("lido_calculator_state", b"state")]
    );
}

pub mod initial_manager {
    sanctum_macros::declare_program_keys!("CK9cEJT7K7oRrMCcEbBQRGqHLGpxKXWnKvW7nHSDMHD1", []);
}

pub struct LidoSolValCalc;

impl GenericPoolSolValCalc for LidoSolValCalc {
    const POOL_PROGRAM_ID: Pubkey = lido_program::ID;
    const POOL_PROGRAM_PROGDATA_ID: Pubkey = lido_program_progdata::ID;
    const CALCULATOR_STATE_PDA: Pubkey = program::LIDO_CALCULATOR_STATE_ID;
    const CALCULATOR_STATE_BUMP: u8 = program::LIDO_CALCULATOR_STATE_BUMP;
    const ID: Pubkey = program::ID;
}

mod account_resolvers {
    use generic_pool_calculator_interface::LST_TO_SOL_IX_ACCOUNTS_LEN;
    use generic_pool_calculator_lib::account_resolvers::LstSolCommonIntermediateKeys;
    use lido_keys::{lido_state, stsol};
    use solana_program::instruction::AccountMeta;

    use crate::LidoSolValCalc;

    pub const LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS: LstSolCommonIntermediateKeys =
        LstSolCommonIntermediateKeys {
            lst_mint: stsol::ID,
            pool_state: lido_state::ID,
        };

    pub fn lido_sol_val_calc_account_metas() -> [AccountMeta; LST_TO_SOL_IX_ACCOUNTS_LEN] {
        let lido_sol_val_calc_keys: generic_pool_calculator_interface::SolToLstKeys =
            LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS
                .resolve::<LidoSolValCalc>()
                .into();
        lido_sol_val_calc_keys.into()
    }
}
