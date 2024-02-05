use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_controller_interface::add_lst_ix_with_program_id;
use s_controller_lib::{find_pool_state_address, AddLstFreeArgs};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};
use std::str::FromStr;

use crate::common::{find_sanctum_lst, sol_val_calc_of_sanctum_lst};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Add a new LST to the pool")]
pub struct AddLstArgs {
    #[arg(
        long,
        short,
        help = "The pool's admin. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,

    #[arg(
        long,
        short,
        help = "The LST's SOL value calculator program. Required if LST is not on sanctum-lst-list",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub sol_val_calc: Option<Pubkey>,

    #[arg(
        long,
        short,
        help = "Mint of the new LST to add",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub mint: Pubkey,
}

impl AddLstArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            admin,
            sol_val_calc,
            mint,
        } = match args.subcmd {
            Subcmd::AddLst(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let admin_signer = admin.map(|s| parse_signer(&s).unwrap());
        let admin = admin_signer.as_ref().unwrap_or(&payer);

        let sol_val_calc = sol_val_calc.unwrap_or_else(|| {
            let sanctum_lst = find_sanctum_lst(mint)
                .expect("LST not found on list, --sol-val-calc must be provided");
            sol_val_calc_of_sanctum_lst(sanctum_lst)
        });

        let pool_state_addr = find_pool_state_address(program_id).0;
        let mut fetched_accs = rpc
            .get_multiple_accounts(&[pool_state_addr, mint])
            .await
            .unwrap();
        let lst_mint_acc = fetched_accs.pop().unwrap().unwrap();
        let pool_state_acc = fetched_accs.pop().unwrap().unwrap();

        let (keys, _bumps) = AddLstFreeArgs {
            payer: payer.pubkey(),
            sol_value_calculator: sol_val_calc,
            pool_state: Keyed {
                pubkey: pool_state_addr,
                account: pool_state_acc,
            },
            lst_mint: Keyed {
                pubkey: mint,
                account: lst_mint_acc,
            },
        }
        .resolve()
        .unwrap();
        let ix = add_lst_ix_with_program_id(program_id, keys).unwrap();

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
