use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_controller_interface::{disable_lst_input_ix_with_program_id, DisableLstInputIxArgs};
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, DisableEnableLstInputByMintFreeArgs,
};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};
use std::str::FromStr;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Disables all input for a LST for a pool.")]
pub struct DisableLstInputArgs {
    #[arg(
        long,
        short,
        help = "The pool's admin. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,

    #[arg(
        help = "Mint of the LST to disable input of",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub mint: Pubkey,
}

impl DisableLstInputArgs {
    pub async fn run(args: crate::Args) {
        let Self { admin, mint } = match args.subcmd {
            Subcmd::DisableLstInput(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let admin_signer = admin.map(|s| parse_signer(&s).unwrap());
        let admin = admin_signer.as_ref().unwrap_or(&payer);

        let pool_state_addr = find_pool_state_address(program_id).0;
        let lst_state_list_addr = find_lst_state_list_address(program_id).0;
        let mut fetched_accs = rpc
            .get_multiple_accounts(&[pool_state_addr, lst_state_list_addr])
            .await
            .unwrap();
        let lst_state_list_acc = fetched_accs.pop().unwrap().unwrap();
        let pool_state_acc = fetched_accs.pop().unwrap().unwrap();

        let (keys, index) = DisableEnableLstInputByMintFreeArgs {
            lst_mint: mint,
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
        }
        .resolve_disable_for_prog(program_id)
        .unwrap();
        let ix = disable_lst_input_ix_with_program_id(
            program_id,
            keys,
            DisableLstInputIxArgs {
                index: index.try_into().unwrap(),
            },
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
