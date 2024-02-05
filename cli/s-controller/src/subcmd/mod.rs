use clap::Subcommand;

use self::{
    add_disable_auth::AddDisableAuthArgs, disable_pool::DisablePoolArgs, init::InitArgs,
    set_admin::SetAdminArgs,
};

mod add_disable_auth;
mod disable_pool;
mod init;
mod set_admin;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init(InitArgs),
    AddDisableAuth(AddDisableAuthArgs),
    SetAdmin(SetAdminArgs),
    DisablePool(DisablePoolArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match args.subcmd {
            Self::Init(_) => InitArgs::run(args).await,
            Self::AddDisableAuth(_) => AddDisableAuthArgs::run(args).await,
            Self::SetAdmin(_) => SetAdminArgs::run(args).await,
            Self::DisablePool(_) => DisablePoolArgs::run(args).await,
        }
    }
}
