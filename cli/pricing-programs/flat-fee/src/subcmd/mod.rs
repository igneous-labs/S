use clap::Subcommand;

mod add_lst;
mod common;
mod initialize;
mod remove_lst;
mod set_lst_fee;
mod set_manager;

use add_lst::AddLstArgs;
use initialize::InitializeArgs;
use remove_lst::RemoveLstArgs;
use set_lst_fee::SetLstFeeArgs;
use set_manager::SetManagerArgs;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Initialize,
    SetManager(SetManagerArgs),
    AddLst(AddLstArgs),
    RemoveLst(RemoveLstArgs),
    SetLstFee(SetLstFeeArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Initialize => InitializeArgs::run(args).await,
            Self::SetManager(_) => SetManagerArgs::run(args).await,
            Self::AddLst(_) => AddLstArgs::run(args).await,
            Self::RemoveLst(_) => RemoveLstArgs::run(args).await,
            Self::SetLstFee(_) => SetLstFeeArgs::run(args).await,
        }
    }
}
