use clap::Args;
use flat_fee_interface::{
    price_exact_out_ix_with_program_id, PriceExactOutIxArgs, PriceExactOutKeys,
};
use flat_fee_lib::pda::FeeAccountFindPdaArgs;
use solana_sdk::native_token::sol_to_lamports;

use crate::lst_arg::LstArg;

use super::{common::handle_pricing_ix, Subcmd};

#[derive(Args, Debug)]
#[command(
    long_about = "Simulates PriceExactOut and outputs the input SOL value return value to stdout as a single decimal value"
)]
pub struct PriceExactOutArgs {
    #[arg(
        long,
        short,
        default_value_t = 1.0,
        help = "Output LST decimal amount. Defaults to 1.0 if not set."
    )]
    pub amount: f64,

    #[arg(
        long,
        short,
        default_value_t = 1.0,
        help = "SOL value of `amount` output LST. Defaults to 1.0 if not set."
    )]
    pub sol_value: f64,

    #[arg(
        help = "Input LST. Can either be a pubkey or case-sensitive symbol of a token on sanctum-lst-list. e.g. 'bSOL'"
    )]
    pub input: String,

    #[arg(
        help = "Output LST. Can either be a pubkey or case-sensitive symbol of a token on sanctum-lst-list. e.g. 'bSOL'"
    )]
    pub output: String,
}

impl PriceExactOutArgs {
    pub async fn run(args: crate::Args) {
        let slsts = args.load_slst_list();
        let Self {
            amount,
            sol_value,
            input,
            output,
        } = match args.subcmd {
            Subcmd::PriceExactOut(a) => a,
            _ => unreachable!(),
        };
        let [input, output] = [input, output].map(|a| LstArg::parse_arg(&a, &slsts).unwrap());
        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let input_lst_mint = input.mint();
        let input_fee_acc = FeeAccountFindPdaArgs {
            program_id,
            lst_mint: input_lst_mint,
        }
        .get_fee_account_address_and_bump_seed()
        .0;
        let output_lst_mint = output.mint();
        let output_fee_acc = FeeAccountFindPdaArgs {
            program_id,
            lst_mint: output_lst_mint,
        }
        .get_fee_account_address_and_bump_seed()
        .0;
        let ix = price_exact_out_ix_with_program_id(
            program_id,
            PriceExactOutKeys {
                input_lst_mint,
                output_lst_mint,
                input_fee_acc,
                output_fee_acc,
            },
            PriceExactOutIxArgs {
                amount: sol_to_lamports(amount),
                sol_value: sol_to_lamports(sol_value),
            },
        )
        .unwrap();

        handle_pricing_ix(&rpc, ix, payer.as_ref()).await;
    }
}
