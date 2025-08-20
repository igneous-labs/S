use std::{path::PathBuf, str::FromStr};

use clap::{
    builder::{StringValueParser, TypedValueParser, ValueParser},
    Parser,
};
use s_cli_utils::{CONFIG_HELP, FEE_LIMIT_CB_HELP, TX_SEND_MODE_HELP};
use sanctum_lst_list::{SanctumLst, SanctumLstList};
use sanctum_solana_cli_utils::{ConfigWrapper, TxSendMode};
use solana_sdk::pubkey::Pubkey;
use subcmd::Subcmd;
use tokio::runtime::Runtime;

mod lst_arg;
mod subcmd;

#[derive(Parser, Debug)]
#[command(author, version, about = "Flat-Fee Pricing Program CLI")]
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
        help = "program ID of the flat-fee pricing program",
        default_value_t = flat_fee_lib::program::ID,
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub program: Pubkey,

    #[arg(
        long,
        short,
        help = FEE_LIMIT_CB_HELP,
        default_value_t = 1
    )]
    pub fee_limit_cb: u64,

    #[arg(long, short = 'a', help = "Path to sanctum-lst-list.toml")]
    pub sanctum_lst_list: Option<PathBuf>,

    #[command(subcommand)]
    pub subcmd: Subcmd,
}

impl Args {
    pub fn load_slst_list(&self) -> Vec<SanctumLst> {
        self.sanctum_lst_list
            .as_ref()
            .map_or_else(SanctumLstList::load, |p| {
                SanctumLstList::load_from_file(p)
                    .map_err(|e| format!("Could not load sanctum-lst-list: {e}"))
                    .unwrap()
            })
            .sanctum_lst_list
    }
}

fn main() {
    let args = Args::parse();
    let rt = Runtime::new().unwrap();
    rt.block_on(Subcmd::run(args));
}
