use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_cli_utils::handle_tx_full;
use s_controller_interface::add_disable_pool_authority_ix_with_program_id;
use s_controller_lib::{find_pool_state_address, try_pool_state, AddDisablePoolAuthorityFreeArgs};
use sanctum_solana_cli_utils::parse_signer;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::pubkey::Pubkey;

use crate::{common::verify_admin, rpc::fetch_pool_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    long_about = "Add an authority allowed to disable the entire pool to the list of authorities"
)]
pub struct AddDisableAuthArgs {
    #[arg(
        long,
        short,
        help = "The pool's admin. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,

    #[arg(
        help = "The authority to add",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub new_authority: Pubkey,
}

impl AddDisableAuthArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            admin,
            new_authority,
        } = match args.subcmd {
            Subcmd::AddDisableAuth(a) => a,
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

        let ix = add_disable_pool_authority_ix_with_program_id(
            program_id,
            AddDisablePoolAuthorityFreeArgs {
                payer: payer.pubkey(),
                new_authority,
                pool_state_acc: KeyedAccount {
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
            &mut [payer.as_ref(), admin.as_ref()],
        )
        .await;
    }
}
