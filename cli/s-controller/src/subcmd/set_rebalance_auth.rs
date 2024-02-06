use clap::Args;
use s_controller_interface::set_rebalance_authority_ix_with_program_id;
use s_controller_lib::{try_pool_state, KnownAuthoritySetRebalanceAuthorityFreeArgs};
use sanctum_solana_cli_utils::{parse_pubkey_src, parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use crate::rpc::fetch_pool_state;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Sets the S controller program's rebalance authority.")]
pub struct SetRebalanceAuthArgs {
    #[arg(
        long,
        short,
        help = "Either the program's admin authority signer or the rebalance authority. Defaults to config wallet if not set."
    )]
    pub authority: Option<String>,

    #[arg(help = "The new rebalance authority to set to. Can be a pubkey or signer.")]
    pub new_rebalance_auth: String,
}

impl SetRebalanceAuthArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            authority,
            new_rebalance_auth,
        } = match args.subcmd {
            Subcmd::SetRebalanceAuth(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let authority_signer = authority.map(|s| parse_signer(&s).unwrap());
        let authority = authority_signer.as_ref().unwrap_or(&payer);
        let new_rebalance_authority = parse_pubkey_src(&new_rebalance_auth).unwrap().pubkey();

        let pool_state_acc = fetch_pool_state(&rpc, program_id).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        let keys = if pool_state.admin == authority.pubkey() {
            KnownAuthoritySetRebalanceAuthorityFreeArgs {
                new_rebalance_authority,
                pool_state: pool_state_acc,
            }
            .resolve_pool_admin_for_prog(program_id)
            .unwrap()
        } else if pool_state.rebalance_authority == authority.pubkey() {
            KnownAuthoritySetRebalanceAuthorityFreeArgs {
                new_rebalance_authority,
                pool_state: pool_state_acc,
            }
            .resolve_current_rebalance_authority_for_prog(program_id)
            .unwrap()
        } else {
            eprintln!(
                "{} not authorized to change rebalance authority",
                authority.pubkey()
            );
            std::process::exit(-1);
        };

        let ix = set_rebalance_authority_ix_with_program_id(program_id, keys).unwrap();

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
