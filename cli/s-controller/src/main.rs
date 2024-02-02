use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser, ValueParser},
    Parser,
};
use s_cli_utils::{CONFIG_HELP, TX_SEND_MODE_HELP};
use sanctum_solana_cli_utils::{ConfigWrapper, TxSendMode};
use solana_sdk::pubkey::Pubkey;

mod subcmd;

#[derive(Parser, Debug)]
#[command(author, version, about = "S Controller Program CLI")]
struct Args {
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
        long,
        short,
        help = "program ID of the S controlker program",
        default_value_t = s_controller_lib::program::ID,
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub program: Pubkey,
    //#[command(subcommand)]
    //pub subcmd: Subcmd,
}

fn main() {
    let _args = Args::parse();
}
