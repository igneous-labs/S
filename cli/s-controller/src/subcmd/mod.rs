use clap::Subcommand;

mod init;

#[derive(Debug, Subcommand)]
enum Subcmd {
    Init,
}

/*
impl Subcmd {
    async fn run(_args: crate::Args) {
        todo!()
    }
}
 */
