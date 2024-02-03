use clap::Subcommand;

use self::{init::InitArgs, set_admin::SetAdminArgs};

mod common;
mod init;
mod set_admin;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init(InitArgs),
    SetAdmin(SetAdminArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match args.subcmd {
            Self::Init(_) => InitArgs::run(args).await,
            Self::SetAdmin(_) => SetAdminArgs::run(args).await,
        }
    }
}
