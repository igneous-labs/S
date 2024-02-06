use clap::Subcommand;

use self::{
    add_disable_auth::AddDisableAuthArgs, add_lst::AddLstArgs,
    disable_lst_input::DisableLstInputArgs, disable_pool::DisablePoolArgs,
    enable_lst_input::EnableLstInputArgs, enable_pool::EnablePoolArgs, init::InitArgs,
    remove_disable_auth::RemoveDisableAuthArgs, remove_lst::RemoveLstArgs, set_admin::SetAdminArgs,
};

mod add_disable_auth;
mod add_lst;
mod disable_lst_input;
mod disable_pool;
mod enable_lst_input;
mod enable_pool;
mod init;
mod remove_disable_auth;
mod remove_lst;
mod set_admin;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init(InitArgs),
    AddDisableAuth(AddDisableAuthArgs),
    RemoveDisableAuth(RemoveDisableAuthArgs),
    SetAdmin(SetAdminArgs),
    AddLst(AddLstArgs),
    RemoveLst(RemoveLstArgs),
    DisableLstInput(DisableLstInputArgs),
    EnableLstInput(EnableLstInputArgs),
    DisablePool(DisablePoolArgs),
    EnablePool(EnablePoolArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match args.subcmd {
            Self::Init(_) => InitArgs::run(args).await,
            Self::AddDisableAuth(_) => AddDisableAuthArgs::run(args).await,
            Self::RemoveDisableAuth(_) => RemoveDisableAuthArgs::run(args).await,
            Self::SetAdmin(_) => SetAdminArgs::run(args).await,
            Self::AddLst(_) => AddLstArgs::run(args).await,
            Self::RemoveLst(_) => RemoveLstArgs::run(args).await,
            Self::DisableLstInput(_) => DisableLstInputArgs::run(args).await,
            Self::EnableLstInput(_) => EnableLstInputArgs::run(args).await,
            Self::DisablePool(_) => DisablePoolArgs::run(args).await,
            Self::EnablePool(_) => EnablePoolArgs::run(args).await,
        }
    }
}
