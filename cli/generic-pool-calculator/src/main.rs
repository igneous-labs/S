use clap::{
    builder::{StringValueParser, TypedValueParser, ValueParser},
    Parser,
};
use s_cli_utils::{CONFIG_HELP, FEE_LIMIT_CB_HELP, TX_SEND_MODE_HELP};
use sanctum_solana_cli_utils::{ConfigWrapper, TxSendMode};
use sol_val_calc_arg::SolValCalcArg;
use tokio::runtime::Runtime;

mod sol_val_calc_arg;
mod subcmd;

use subcmd::Subcmd;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Generic Stake Pool SOL Value Calculator Program CLI"
)]
pub struct Args {
    #[arg(
        long,
        short,
        help = CONFIG_HELP,
        default_value = "",
        value_parser = ValueParser::new(ConfigWrapper::parse_from_path)
    )]
    pub config: ConfigWrapper,

    #[arg(
        long,
        short,
        help = TX_SEND_MODE_HELP,
        default_value_t = TxSendMode::default(),
        value_enum,
    )]
    pub send_mode: TxSendMode,

    #[arg(
        help = SolValCalcArg::HELP_STR,
        value_parser = StringValueParser::new().try_map(|s| SolValCalcArg::parse_arg(&s)),
    )]
    pub program: SolValCalcArg,

    #[arg(
        long,
        short,
        help = FEE_LIMIT_CB_HELP,
        default_value_t = 1
    )]
    pub fee_limit_cb: u64,

    #[command(subcommand)]
    pub subcmd: Subcmd,
}

fn main() {
    let args = Args::parse();
    let rt = Runtime::new().unwrap();
    rt.block_on(Subcmd::run(args));
}
