use clap::Args;
use flat_fee_interface::{
    price_lp_tokens_to_mint_ix_with_program_id, PriceLpTokensToMintIxArgs, PriceLpTokensToMintKeys,
};
use solana_sdk::native_token::sol_to_lamports;

use crate::lst_arg::LstArg;

use super::{common::handle_pricing_ix, Subcmd};

#[derive(Args, Debug)]
#[command(
    long_about = "Simulates PriceLpTokensToMint and outputs the LP tokens SOL value return value to stdout as a single decimal value"
)]
pub struct PriceLpTokensToMintArgs {
    #[arg(
        long,
        short,
        default_value_t = 1.0,
        help = "Deposit LST decimal amount. Defaults to 1.0 if not set."
    )]
    pub amount: f64,

    #[arg(
        long,
        short,
        default_value_t = 1.0,
        help = "SOL value of `amount` LST to deposit. Defaults to 1.0 if not set."
    )]
    pub sol_value: f64,

    #[arg(
        help = "Mint of the LST to deposit. Can either be a pubkey or case-sensitive symbol of a token on sanctum-lst-list. e.g. 'bSOL'"
    )]
    pub lst_mint: String,
}

impl PriceLpTokensToMintArgs {
    pub async fn run(args: crate::Args) {
        let slsts = args.load_slst_list();
        let Self {
            amount,
            sol_value,
            lst_mint,
        } = match args.subcmd {
            Subcmd::PriceLpTokensToMint(a) => a,
            _ => unreachable!(),
        };
        let lst_mint = LstArg::parse_arg(&lst_mint, &slsts).unwrap();
        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let ix = price_lp_tokens_to_mint_ix_with_program_id(
            program_id,
            PriceLpTokensToMintKeys {
                input_lst_mint: lst_mint.mint(),
            },
            PriceLpTokensToMintIxArgs {
                amount: sol_to_lamports(amount),
                sol_value: sol_to_lamports(sol_value),
            },
        )
        .unwrap();

        handle_pricing_ix(&rpc, ix, payer.as_ref()).await;
    }
}
