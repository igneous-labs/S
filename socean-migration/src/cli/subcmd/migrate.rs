use clap::Args;
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use socean_migration::migrate_ix;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::Subcmd;

#[derive(Args, Debug)]
pub struct MigrateArgs {
    #[arg(
        long,
        short,
        help = "The migrate auth. Defaults to config wallet if not set."
    )]
    pub migrate_auth: Option<String>,
}

impl MigrateArgs {
    pub async fn run(args: crate::cli::Args) {
        let Self { migrate_auth } = match args.subcmd {
            Subcmd::Migrate(a) => a,
            _ => unreachable!(),
        };

        let rpc = args.config.nonblocking_rpc_client();
        let payer = args.config.signer();

        let migrate_auth_signer = migrate_auth.map(|s| parse_signer(&s).unwrap());
        let migrate_auth = migrate_auth_signer.as_ref().unwrap_or(&payer);

        let mut signers = vec![payer.as_ref(), migrate_auth.as_ref()];
        signers.dedup();

        let ix = migrate_ix();

        let rbh = rpc.get_latest_blockhash().await.unwrap();
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(&payer.pubkey(), &[ix], &[], rbh).unwrap()),
            &signers,
        )
        .unwrap();

        rpc.handle_tx(&tx, args.send_mode).await;
    }
}
