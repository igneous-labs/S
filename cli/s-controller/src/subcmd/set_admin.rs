use clap::Args;
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use s_controller_interface::set_admin_ix_with_program_id;
use s_controller_lib::{find_pool_state_address, try_pool_state, SetAdminFreeArgs};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_readonly_account::sdk::KeyedAccount;

use crate::{common::verify_admin, rpc::fetch_pool_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Sets the S controller program's admin authority.",
    long_about = "Sets the S controller program's admin authority.

Prerequisites:
- The program's pool state must be initialized prior to the invocation."
)]
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

        let curr_admin_signer =
            curr_admin.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let curr_admin = curr_admin_signer.as_ref().unwrap_or(&payer);
        let new_admin = PubkeySrc::parse(&new_admin).unwrap().pubkey();

        let pool_state_acc = fetch_pool_state(&rpc, program_id).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        verify_admin(pool_state, curr_admin.pubkey()).unwrap();

        let ix = set_admin_ix_with_program_id(
            program_id,
            SetAdminFreeArgs {
                new_admin,
                pool_state: KeyedAccount {
                    pubkey: find_pool_state_address(program_id).0,
                    account: pool_state_acc,
                },
            }
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
            &mut [payer.as_ref(), curr_admin.as_ref()],
        )
        .await;
    }
}
