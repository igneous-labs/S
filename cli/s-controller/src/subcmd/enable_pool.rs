use clap::Args;
use s_controller_interface::enable_pool_ix_with_program_id;
use s_controller_lib::{try_pool_state, EnablePoolFreeArgs};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use crate::{common::verify_admin, rpc::fetch_pool_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "(Re-)enables functionality of the entire pool")]
pub struct EnablePoolArgs {
    #[arg(
        long,
        short,
        help = "The program's admin. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,
}

impl EnablePoolArgs {
    pub async fn run(args: crate::Args) {
        let Self { admin } = match args.subcmd {
            Subcmd::EnablePool(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let admin_signer = admin.map(|s| parse_signer(&s).unwrap());
        let admin = admin_signer.as_ref().unwrap_or(&payer);

        let pool_state_acc = fetch_pool_state(&rpc, program_id).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

        verify_admin(pool_state, admin.pubkey()).unwrap();

        let ix = enable_pool_ix_with_program_id(
            program_id,
            EnablePoolFreeArgs { pool_state_acc }
                .resolve_for_prog(program_id)
                .unwrap(),
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), admin.as_ref()];
        signers.dedup();

        let rbh = rpc.get_latest_blockhash().await.unwrap();
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(&payer.pubkey(), &[ix], &[], rbh).unwrap()),
            &signers,
        )
        .unwrap();

        rpc.handle_tx(&tx, args.send_mode).await;
    }
}
