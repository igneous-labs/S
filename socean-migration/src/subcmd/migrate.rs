use clap::Args;

#[derive(Args, Debug)]
pub struct MigrateArgs {}

impl MigrateArgs {
    pub async fn run(_args: crate::Args) {
        todo!();
    }
}
