use clap::Subcommand;

use self::init::InitArgs;

mod init;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init(InitArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match args.subcmd {
            Self::Init(_) => InitArgs::run(args).await,
        }
    }
}
