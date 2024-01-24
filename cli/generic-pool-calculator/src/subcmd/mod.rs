use clap::Subcommand;

mod init;
mod set_manager;

use init::InitArgs;

use self::set_manager::SetManagerArgs;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init,
    SetManager(SetManagerArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Init => InitArgs::run(args).await,
            Self::SetManager(_) => SetManagerArgs::run(args).await,
        }
    }
}
