use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use sanctum_solana_cli_utils::parse_signer;

use crate::{lst_amt_arg::LstAmtArg, lst_arg::LstArg};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Rebalance from SOL to another LST",
    long_about = "Rebalance from SOL to another LST.
May require the payer to subsidize some amount of LST to make up for the stake pool's SOL deposit fees"
)]
pub struct RebalSolArgs {
    #[arg(
        long,
        short,
        help = "The program's rebalance authority. Defaults to config wallet if not set."
    )]
    pub rebalance_auth: Option<String>,

    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Auto-confirm subsidy amount"
    )]
    pub yes: bool,

    #[arg(
        long,
        short,
        default_value_t = LstAmtArg::All,
        help = "Amount in SOL to rebalance",
        value_parser = StringValueParser::new().try_map(|s| LstAmtArg::parse_arg(&s)),
    )]
    pub sol: LstAmtArg,

    #[arg(
        help = "The LST to rebalance SOL into",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub lst: LstArg,
}

impl RebalSolArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            rebalance_auth,
            yes,
            sol,
            lst,
        } = match args.subcmd {
            Subcmd::RebalSol(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let rebalance_auth = rebalance_auth.map(|s| parse_signer(&s).unwrap());
        let rebalance_auth = rebalance_auth
            .as_ref()
            .map_or_else(|| payer.as_ref(), |s| s.as_ref());

        /*
        let amt = match sol {
            LstAmtArg::Amt(v) => v,
            Lst
        };
         */
    }
}
