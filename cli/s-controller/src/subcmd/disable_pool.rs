use clap::Args;
use s_controller_interface::disable_pool_ix_with_program_id;
use s_controller_lib::{try_disable_pool_authority_list, try_pool_state, DisablePoolFreeArgs};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use crate::{
    common::verify_disable_pool_authority,
    rpc::{fetch_disable_pool_authority_list, fetch_pool_state},
};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Disables functionality of the entire pool.

Prerequisites:
- The program's pool state must be initialized prior to the invokation.
")]
pub struct DisablePoolArgs {
    #[arg(
        long,
        short,
        help = "The program's admin or a disable pool authority signer. Defaults to config wallet if not set."
    )]
    pub authority: Option<String>,
}

impl DisablePoolArgs {
    pub async fn run(args: crate::Args) {
        let Self { authority } = match args.subcmd {
            Subcmd::DisablePool(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let authority_signer = authority.map(|s| parse_signer(&s).unwrap());
        let authority = authority_signer.as_ref().unwrap_or(&payer);

        let pool_state_acc = fetch_pool_state(&rpc, program_id).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();

        // check if authority is either the admin or a disable pool authority
        if pool_state.admin != authority.pubkey() {
            let disable_pool_authority_list_acc =
                fetch_disable_pool_authority_list(&rpc, program_id).await;
            let disable_pool_authority_list =
                try_disable_pool_authority_list(&disable_pool_authority_list_acc.data).unwrap();

            verify_disable_pool_authority(disable_pool_authority_list, authority.pubkey()).unwrap();
        }

        let ix = disable_pool_ix_with_program_id(
            program_id,
            DisablePoolFreeArgs {
                signer: authority.pubkey(),
            }
            .resolve_for_prog(program_id),
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), authority.as_ref()];
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
