use clap::Subcommand;

use self::{migrate::MigrateArgs, remove_stake::RemoveStakeArgs};

mod migrate;
mod remove_stake;

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    Migrate(MigrateArgs),
    RemoveStake(RemoveStakeArgs),
}

impl Subcmd {
    pub async fn run(args: crate::Args) {
        match &args.subcmd {
            Self::Migrate(_) => MigrateArgs::run(args).await,
            Self::RemoveStake(_) => RemoveStakeArgs::run(args).await,
        }
    }
}
