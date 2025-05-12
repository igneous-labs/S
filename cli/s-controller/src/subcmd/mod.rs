use clap::Subcommand;
use rebal_stake::RebalStakeArgs;
use rebal_withdraw_sol::RebalWithdrawSolArgs;

use self::{
    add_disable_auth::AddDisableAuthArgs, add_lst::AddLstArgs,
    disable_lst_input::DisableLstInputArgs, disable_pool::DisablePoolArgs,
    enable_lst_input::EnableLstInputArgs, enable_pool::EnablePoolArgs, init::InitArgs,
    rebal_sol::RebalSolArgs, remove_disable_auth::RemoveDisableAuthArgs, remove_lst::RemoveLstArgs,
    set_admin::SetAdminArgs, set_pricing_prog::SetPricingProgArgs,
    set_protocol_fee::SetProtocolFeeArgs,
    set_protocol_fee_beneficiary::SetProtocolFeeBeneficiaryArgs,
    set_rebalance_auth::SetRebalanceAuthArgs, set_sol_value_calculator::SetSolValueCalculatorArgs,
    sync::SyncArgs, sync_all::SyncAllArgs, view::ViewArgs,
    withdraw_protocol_fees::WithdrawProtocolFeesArgs,
};

mod add_disable_auth;
mod add_lst;
mod disable_lst_input;
mod disable_pool;
mod enable_lst_input;
mod enable_pool;
mod init;
mod rebal_sol;
mod rebal_stake;
mod rebal_withdraw_sol;
mod remove_disable_auth;
mod remove_lst;
mod set_admin;
mod set_pricing_prog;
mod set_protocol_fee;
mod set_protocol_fee_beneficiary;
mod set_rebalance_auth;
mod set_sol_value_calculator;
mod sync;
mod sync_all;
mod view;
mod withdraw_protocol_fees;

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
    Sync(SyncArgs),
    SyncAll(SyncAllArgs),
    WithdrawProtocolFees(WithdrawProtocolFeesArgs),
    View(ViewArgs),
    RebalSol(RebalSolArgs),
    RebalStake(RebalStakeArgs),
    RebalWithdrawSol(RebalWithdrawSolArgs),
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
            Self::Sync(_) => SyncArgs::run(args).await,
            Self::SyncAll(_) => SyncAllArgs::run(args).await,
            Self::WithdrawProtocolFees(_) => WithdrawProtocolFeesArgs::run(args).await,
            Self::View(_) => ViewArgs::run(args).await,
            Self::RebalSol(_) => RebalSolArgs::run(args).await,
            Self::RebalStake(_) => RebalStakeArgs::run(args).await,
            Self::RebalWithdrawSol(_) => RebalWithdrawSolArgs::run(args).await,
        }
    }
}
