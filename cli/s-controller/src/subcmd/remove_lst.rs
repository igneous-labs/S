use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use s_controller_interface::remove_lst_ix_with_program_id;
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, RemoveLstByMintFreeArgs,
};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Removes a LST from the pool",
    long_about = "Removes a LST from the pool.

Prerequisites:
- protocol fee accumulator token account must be empty
- pool reserves token account must be empty

Recommended to disable LST input first to facilitate achieving these prerequisites."
)]
pub struct RemoveLstArgs {
    #[arg(
        long,
        short,
        help = "The pool's admin. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,

    #[arg(
        long,
        short,
        help = "The account to refund rent SOL to. Defaults to config wallet if not set.",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub refund_rent_to: Option<Pubkey>,

    #[arg(
        help = "Mint of the LST to remove",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub mint: Pubkey,
}

impl RemoveLstArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            admin,
            refund_rent_to,
            mint,
        } = match args.subcmd {
            Subcmd::RemoveLst(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let admin_signer =
            admin.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let admin = admin_signer.as_ref().unwrap_or(&payer);
        let refund_rent_to = refund_rent_to.unwrap_or_else(|| payer.pubkey());

        let pool_state_addr = find_pool_state_address(program_id).0;
        let lst_state_list_addr = find_lst_state_list_address(program_id).0;
        let mut fetched_accs = rpc
            .get_multiple_accounts(&[mint, pool_state_addr, lst_state_list_addr])
            .await
            .unwrap();
        let lst_state_list_acc = fetched_accs.pop().unwrap().unwrap();
        let pool_state_acc = fetched_accs.pop().unwrap().unwrap();
        let mint_acc = fetched_accs.pop().unwrap().unwrap();

        let (keys, ix_args) = RemoveLstByMintFreeArgs {
            refund_rent_to,
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
            lst_mint: KeyedAccount {
                pubkey: mint,
                account: mint_acc,
            },
        }
        .resolve_for_prog(program_id)
        .unwrap();
        let ix = remove_lst_ix_with_program_id(program_id, keys, ix_args).unwrap();

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
