use clap::Subcommand;

mod common;
mod initialize;
mod set_manager;

use initialize::InitializeArgs;
use set_manager::SetManagerArgs;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Initialize,
    SetManager(SetManagerArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Initialize => InitializeArgs::run(args).await,
            Self::SetManager(_) => SetManagerArgs::run(args).await,
        }
    }
}
