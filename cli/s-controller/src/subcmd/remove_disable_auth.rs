use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_controller_interface::remove_disable_pool_authority_ix_with_program_id;
use s_controller_lib::{
    find_disable_pool_authority_list_address, find_pool_state_address,
    try_disable_pool_authority_list, try_pool_state, RemoveDisablePoolAuthorityByPubkeyFreeArgs,
};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_readonly_account::ReadonlyAccountData;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};

use crate::common::verify_disable_pool_authority;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    long_about = "Removes an authority allowed to disable the entire pool from the list of authorities"
)]
pub struct RemoveDisableAuthArgs {
    #[arg(
        long,
        short,
        help = "The program's admin or the disable pool authority that's removing itself. Defaults to config wallet if not set."
    )]
    pub authority: Option<String>,

    #[arg(
        long,
        short,
        help = "The account to refund rent SOL to. Defaults to config wallet if not set.",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub refund_rent_to: Option<Pubkey>,

    #[arg(
        help = "The authority to remove",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub removing: Pubkey,
}

impl RemoveDisableAuthArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            authority,
            refund_rent_to,
            removing,
        } = match args.subcmd {
            Subcmd::RemoveDisableAuth(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let authority_signer = authority.map(|s| parse_signer(&s).unwrap());
        let authority = authority_signer.as_ref().unwrap_or(&payer);
        let refund_rent_to = refund_rent_to.unwrap_or_else(|| payer.pubkey());

        let pool_state_addr = find_pool_state_address(program_id).0;
        let disable_auth_list_addr = find_disable_pool_authority_list_address(program_id).0;
        let mut fetched_accs = rpc
            .get_multiple_accounts(&[pool_state_addr, disable_auth_list_addr])
            .await
            .unwrap();
        let disable_auth_list_acc = fetched_accs.pop().unwrap().unwrap();
        let pool_state_acc = fetched_accs.pop().unwrap().unwrap();

        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        let disable_auth_list =
            try_disable_pool_authority_list(&disable_auth_list_acc.data()).unwrap();

        // check if authority is either the admin or a disable pool authority
        if pool_state.admin != authority.pubkey() {
            verify_disable_pool_authority(disable_auth_list, authority.pubkey()).unwrap();
            if authority.pubkey() != removing {
                eprintln!("A non-admin authority can only remove itself from the list");
                std::process::exit(-1);
            }
        }

        let (keys, ix_args) = RemoveDisablePoolAuthorityByPubkeyFreeArgs {
            refund_rent_to,
            signer: authority.pubkey(),
            authority: removing,
            pool_state_acc,
            disable_pool_authority_list: disable_auth_list_acc,
        }
        .resolve_for_prog(program_id)
        .unwrap();

        let ix =
            remove_disable_pool_authority_ix_with_program_id(program_id, keys, ix_args).unwrap();

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
