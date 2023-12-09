use sol_value_calculator_lib::SolValueCalculator;
use solana_program::program_error::ProgramError;

pub use account_resolvers::*;

pub mod program {
    pub const ID: solana_program::pubkey::Pubkey = wsol_calculator_interface::ID;
}

pub struct WsolSolCalc;

impl SolValueCalculator for WsolSolCalc {
    fn calc_lst_to_sol(&self, lst_amount: u64) -> Result<u64, ProgramError> {
        Ok(lst_amount)
    }

    fn calc_sol_to_lst(&self, lamports_amount: u64) -> Result<u64, ProgramError> {
        Ok(lamports_amount)
    }
}

mod account_resolvers {
    use solana_program::instruction::AccountMeta;
    use wsol_calculator_interface::{LstToSolKeys, SolToLstKeys};

    pub const WSOL_LST_TO_SOL_KEYS: LstToSolKeys = LstToSolKeys {
        lst_mint: wsol_keys::wsol::ID,
    };

    pub const WSOL_SOL_TO_LST_KEYS: SolToLstKeys = SolToLstKeys {
        lst_mint: wsol_keys::wsol::ID,
    };

    pub const WSOL_LST_TO_SOL_METAS: [AccountMeta; 1] = [AccountMeta {
        pubkey: wsol_keys::wsol::ID,
        is_signer: false,
        is_writable: false,
    }];

    pub const WSOL_SOL_TO_LST_METAS: [AccountMeta; 1] = WSOL_LST_TO_SOL_METAS;
}
