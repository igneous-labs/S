use clap::Subcommand;

use self::{
    add_disable_auth::AddDisableAuthArgs, add_lst::AddLstArgs,
    disable_lst_input::DisableLstInputArgs, disable_pool::DisablePoolArgs,
    enable_lst_input::EnableLstInputArgs, enable_pool::EnablePoolArgs, init::InitArgs,
    set_admin::SetAdminArgs, set_protocol_fee::SetProtocolFeeArgs,
    set_protocol_fee_beneficiary::SetProtocolFeeBeneficiaryArgs,
};

mod add_disable_auth;
mod add_lst;
mod disable_lst_input;
mod disable_pool;
mod enable_lst_input;
mod enable_pool;
mod init;
mod set_admin;
mod set_protocol_fee;
mod set_protocol_fee_beneficiary;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init(InitArgs),
    AddDisableAuth(AddDisableAuthArgs),
    SetAdmin(SetAdminArgs),
    SetProtocolFee(SetProtocolFeeArgs),
    AddLst(AddLstArgs),
    DisableLstInput(DisableLstInputArgs),
    EnableLstInput(EnableLstInputArgs),
    DisablePool(DisablePoolArgs),
    SetProtocolFeeBeneficiary(SetProtocolFeeBeneficiaryArgs),
    EnablePool(EnablePoolArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match args.subcmd {
            Self::Init(_) => InitArgs::run(args).await,
            Self::AddDisableAuth(_) => AddDisableAuthArgs::run(args).await,
            Self::SetAdmin(_) => SetAdminArgs::run(args).await,
            Self::SetProtocolFee(_) => SetProtocolFeeArgs::run(args).await,
            Self::AddLst(_) => AddLstArgs::run(args).await,
            Self::DisableLstInput(_) => DisableLstInputArgs::run(args).await,
            Self::EnableLstInput(_) => EnableLstInputArgs::run(args).await,
            Self::DisablePool(_) => DisablePoolArgs::run(args).await,
            Self::SetProtocolFeeBeneficiary(_) => SetProtocolFeeBeneficiaryArgs::run(args).await,
            Self::EnablePool(_) => EnablePoolArgs::run(args).await,
        }
    }
}
