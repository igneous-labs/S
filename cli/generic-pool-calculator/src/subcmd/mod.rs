use clap::Subcommand;

mod init;

use init::InitArgs;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init,
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Init => InitArgs::run(args).await,
        }
    }
}
