use clap::Subcommand;

mod add_lst;
mod common;
mod initialize;
mod remove_lst;
mod set_lp_withdrawal_fee;
mod set_lst_fee;
mod set_manager;
mod view;
mod view_lst;

use add_lst::AddLstArgs;
use initialize::InitializeArgs;
use remove_lst::RemoveLstArgs;
use set_lp_withdrawal_fee::SetLpWithdrawalFeeArgs;
use set_lst_fee::SetLstFeeArgs;
use set_manager::SetManagerArgs;

use self::{view::ViewArgs, view_lst::ViewLstArgs};

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Initialize,
    SetManager(SetManagerArgs),
    AddLst(AddLstArgs),
    RemoveLst(RemoveLstArgs),
    SetLstFee(SetLstFeeArgs),
    SetLpWithdrawalFee(SetLpWithdrawalFeeArgs),
    View(ViewArgs),
    ViewLst(ViewLstArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Initialize => InitializeArgs::run(args).await,
            Self::SetManager(_) => SetManagerArgs::run(args).await,
            Self::AddLst(_) => AddLstArgs::run(args).await,
            Self::RemoveLst(_) => RemoveLstArgs::run(args).await,
            Self::SetLstFee(_) => SetLstFeeArgs::run(args).await,
            Self::SetLpWithdrawalFee(_) => SetLpWithdrawalFeeArgs::run(args).await,
            Self::View(_) => ViewArgs::run(args).await,
            Self::ViewLst(_) => ViewLstArgs::run(args).await,
        }
    }
}
