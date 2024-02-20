//! CLI to run the migration program

use clap::{builder::ValueParser, Parser};
use sanctum_solana_cli_utils::{ConfigWrapper, TxSendMode};
use tokio::runtime::Runtime;

use subcmd::Subcmd;

mod subcmd;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(
        long,
        short,
        help = "Path to solana CLI config. Defaults to solana cli default if not provided",
        default_value = "",
        value_parser = ValueParser::new(ConfigWrapper::parse_from_path)
    )]
    pub config: ConfigWrapper,

    #[arg(
        long,
        short,
        help = "Transaction send mode.
- send-actual: signs and sends the tx to the cluster specified in config and outputs hash to stderr
- sim-only: simulates the tx against the cluster and outputs logs to stderr
- dump-msg: dumps the base64 encoded tx to stdout. For use with inspectors and multisigs",
        default_value_t = TxSendMode::default(),
        value_enum,
    )]
    pub send_mode: TxSendMode,

    #[command(subcommand)]
    pub subcmd: Subcmd,
}

pub fn main() {
    let args = Args::parse();
    let rt = Runtime::new().unwrap();
    rt.block_on(Subcmd::run(args));
}
