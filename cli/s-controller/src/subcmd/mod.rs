use clap::Subcommand;

use self::{
    add_disable_auth::AddDisableAuthArgs, add_lst::AddLstArgs,
    disable_lst_input::DisableLstInputArgs, disable_pool::DisablePoolArgs,
    enable_lst_input::EnableLstInputArgs, enable_pool::EnablePoolArgs, init::InitArgs,
    remove_disable_auth::RemoveDisableAuthArgs, remove_lst::RemoveLstArgs, set_admin::SetAdminArgs,
    set_pricing_prog::SetPricingProgArgs, set_protocol_fee::SetProtocolFeeArgs,
    set_protocol_fee_beneficiary::SetProtocolFeeBeneficiaryArgs,
    set_rebalance_auth::SetRebalanceAuthArgs, set_sol_value_calculator::SetSolValueCalculatorArgs,
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
mod set_pricing_prog;
mod set_protocol_fee;
mod set_protocol_fee_beneficiary;
mod set_rebalance_auth;
mod set_sol_value_calculator;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init(InitArgs),
    AddDisableAuth(AddDisableAuthArgs),
    RemoveDisableAuth(RemoveDisableAuthArgs),
    SetAdmin(SetAdminArgs),
    SetProtocolFee(SetProtocolFeeArgs),
    AddLst(AddLstArgs),
    RemoveLst(RemoveLstArgs),
    DisableLstInput(DisableLstInputArgs),
    EnableLstInput(EnableLstInputArgs),
    DisablePool(DisablePoolArgs),
    SetProtocolFeeBeneficiary(SetProtocolFeeBeneficiaryArgs),
    EnablePool(EnablePoolArgs),
    SetPricingProg(SetPricingProgArgs),
    SetSolValueCalculator(SetSolValueCalculatorArgs),
    SetRebalanceAuth(SetRebalanceAuthArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match args.subcmd {
            Self::Init(_) => InitArgs::run(args).await,
            Self::AddDisableAuth(_) => AddDisableAuthArgs::run(args).await,
            Self::RemoveDisableAuth(_) => RemoveDisableAuthArgs::run(args).await,
            Self::SetAdmin(_) => SetAdminArgs::run(args).await,
            Self::SetProtocolFee(_) => SetProtocolFeeArgs::run(args).await,
            Self::AddLst(_) => AddLstArgs::run(args).await,
            Self::RemoveLst(_) => RemoveLstArgs::run(args).await,
            Self::DisableLstInput(_) => DisableLstInputArgs::run(args).await,
            Self::EnableLstInput(_) => EnableLstInputArgs::run(args).await,
            Self::DisablePool(_) => DisablePoolArgs::run(args).await,
            Self::SetProtocolFeeBeneficiary(_) => SetProtocolFeeBeneficiaryArgs::run(args).await,
            Self::EnablePool(_) => EnablePoolArgs::run(args).await,
            Self::SetPricingProg(_) => SetPricingProgArgs::run(args).await,
            Self::SetSolValueCalculator(_) => SetSolValueCalculatorArgs::run(args).await,
            Self::SetRebalanceAuth(_) => SetRebalanceAuthArgs::run(args).await,
        }
    }
}
