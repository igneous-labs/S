use clap::Args;
use s_controller_interface::{set_admin_ix_with_program_id, SetAdminKeys};
use s_controller_lib::{find_pool_state_address, try_pool_state};
use sanctum_solana_cli_utils::{parse_pubkey_src, parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::{common::verify_admin, Subcmd};

#[derive(Args, Debug)]
#[command(long_about = "Sets the S controller program's admin authority.

Prerequisites:
- The program's pool state must be initialized prior to the invokation.
")]
pub struct SetAdminArgs {
    #[arg(
        long,
        short,
        help = "The program's admin authority signer. Defaults to config wallet if not set."
    )]
    pub curr_admin: Option<String>,

    #[arg(help = "The new program's admin authority to set. Can be a pubkey or signer.")]
    pub new_admin: String,
}

impl SetAdminArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            curr_admin,
            new_admin,
        } = match args.subcmd {
            Subcmd::SetAdmin(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let curr_admin_signer = curr_admin.map(|s| parse_signer(&s).unwrap());
        let curr_admin = curr_admin_signer.as_ref().unwrap_or(&payer);

        let new_admin = parse_pubkey_src(&new_admin).unwrap();
        let pool_state_pda = find_pool_state_address(program_id).0;
        let pool_state_data = rpc.get_account_data(&pool_state_pda).await.unwrap();
        let pool_state = try_pool_state(&pool_state_data).unwrap();
        verify_admin(pool_state, curr_admin.pubkey()).unwrap();

        let ix = set_admin_ix_with_program_id(
            program_id,
            SetAdminKeys {
                current_admin: pool_state.admin,
                new_admin: new_admin.pubkey(),
                pool_state: pool_state_pda,
            },
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), curr_admin.as_ref()];
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
