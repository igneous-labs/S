use clap::Subcommand;

mod common;
mod init;
mod set_manager;
mod update_last_upgrade_slot;

use init::InitArgs;

use self::{set_manager::SetManagerArgs, update_last_upgrade_slot::UpdateLastUpgradeSlotArgs};

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Init,
    SetManager(SetManagerArgs),
    UpdateLastUpgradeSlot(UpdateLastUpgradeSlotArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Init => InitArgs::run(args).await,
            Self::SetManager(_) => SetManagerArgs::run(args).await,
            Self::UpdateLastUpgradeSlot(_) => UpdateLastUpgradeSlotArgs::run(args).await,
        }
    }
}
