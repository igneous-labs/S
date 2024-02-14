use clap::Args;
use generic_pool_calculator_interface::{
    lst_to_sol_ix_with_program_id, LstToSolIxArgs, LstToSolKeys,
};
use solana_sdk::{native_token::sol_to_lamports, pubkey::Pubkey};

use crate::sol_val_calc_arg::SolValCalcArg;

use super::{
    common::{handle_lst_sol_ix, lst_sol_common_account_metas},
    Subcmd,
};

#[derive(Args, Debug)]
#[command(long_about = "Simulates LstToSol and returns the output to stdout as `min,max`")]
pub struct LstToSolArgs {
    #[arg(
        long,
        short,
        default_value_t = 1.0,
        help = "Amount in SOL to run LstToSol for. Defaults to 1.0 if not set"
    )]
    pub sol: f64,

    #[arg(
        long,
        short,
        help = "The stake pool to run for. Must be provided for spl or sanctum-spl"
    )]
    pub pool: Option<Pubkey>,
}

impl LstToSolArgs {
    pub async fn run(args: crate::Args) {
        let Self { sol, pool } = match args.subcmd {
            Subcmd::LstToSol(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();

        if let SolValCalcArg::Unknown(pk) = args.program {
            eprintln!(
                "Unknown program {pk}. This cmd only works for known SOL value calculator programs"
            );
            return;
        }

        let metas = lst_sol_common_account_metas(&rpc, &args.program, pool).await;

        let mut ix = lst_to_sol_ix_with_program_id(
            args.program.program_id(),
            LstToSolKeys {
                // keys will all be replaced by metas
                lst_mint: Pubkey::default(),
                state: Pubkey::default(),
                pool_state: Pubkey::default(),
                pool_program: Pubkey::default(),
                pool_program_data: Pubkey::default(),
            },
            LstToSolIxArgs {
                amount: sol_to_lamports(sol),
            },
        )
        .unwrap();
        ix.accounts = metas;

        handle_lst_sol_ix(&rpc, ix, payer.as_ref()).await;
    }
}
