use clap::Args;
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use s_controller_interface::enable_pool_ix_with_program_id;
use s_controller_lib::{try_pool_state, EnablePoolFreeArgs};
use sanctum_solana_cli_utils::PubkeySrc;

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

        let admin_signer =
            admin.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
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

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            vec![ix],
            &[],
            &mut [payer.as_ref(), admin.as_ref()],
        )
        .await;
    }
}
