use clap::Subcommand;

mod common;
mod init;
mod lst_to_sol;
mod set_manager;
mod sol_to_lst;
mod update_last_upgrade_slot;
mod view;

use init::InitArgs;

use self::{
    lst_to_sol::LstToSolArgs, set_manager::SetManagerArgs, sol_to_lst::SolToLstArgs,
    update_last_upgrade_slot::UpdateLastUpgradeSlotArgs, view::ViewArgs,
};

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init,
    SetManager(SetManagerArgs),
    UpdateLastUpgradeSlot(UpdateLastUpgradeSlotArgs),
    View(ViewArgs),
    SolToLst(SolToLstArgs),
    LstToSol(LstToSolArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Init => InitArgs::run(args).await,
            Self::SetManager(_) => SetManagerArgs::run(args).await,
            Self::UpdateLastUpgradeSlot(_) => UpdateLastUpgradeSlotArgs::run(args).await,
            Self::View(_) => ViewArgs::run(args).await,
            Self::SolToLst(_) => SolToLstArgs::run(args).await,
            Self::LstToSol(_) => LstToSolArgs::run(args).await,
        }
    }
}
