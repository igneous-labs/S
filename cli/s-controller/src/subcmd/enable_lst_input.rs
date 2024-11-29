use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use s_controller_interface::{enable_lst_input_ix_with_program_id, EnableLstInputIxArgs};
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, DisableEnableLstInputByMintFreeArgs,
};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "(Re-)enable input for a LST for a pool.")]
pub struct EnableLstInputArgs {
    #[arg(
        long,
        short,
        help = "The pool's admin. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,

    #[arg(
        help = "Mint of the LST to (re-)enable input of",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub mint: Pubkey,
}

impl EnableLstInputArgs {
    pub async fn run(args: crate::Args) {
        let Self { admin, mint } = match args.subcmd {
            Subcmd::EnableLstInput(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let admin_signer =
            admin.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
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
        .resolve_enable_for_prog(program_id)
        .unwrap();
        let ix = enable_lst_input_ix_with_program_id(
            program_id,
            keys,
            EnableLstInputIxArgs {
                index: index.try_into().unwrap(),
            },
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
