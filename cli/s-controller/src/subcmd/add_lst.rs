use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use s_controller_interface::add_lst_ix_with_program_id;
use s_controller_lib::{find_pool_state_address, try_pool_state, AddLstFreeArgs};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_readonly_account::{keyed::Keyed, ReadonlyAccountData};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::{common::verify_admin, lst_arg::LstArg};

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
        help = "Mint of the new LST to add. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'"
    )]
    pub mint: String,
}

impl AddLstArgs {
    pub async fn run(args: crate::Args) {
        let slsts = args.load_slst_list();
        let Self {
            admin,
            sol_val_calc,
            mint,
        } = match args.subcmd {
            Subcmd::AddLst(a) => a,
            _ => unreachable!(),
        };
        let mint = LstArg::parse_arg(&mint, &slsts).unwrap();

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let admin_signer =
            admin.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let admin = admin_signer.as_ref().unwrap_or(&payer);

        let sol_val_calc = sol_val_calc.unwrap_or_else(|| {
            mint.sol_val_calc_of()
                .expect("LST not found on list, --sol-val-calc must be provided")
        });

        let pool_state_addr = find_pool_state_address(program_id).0;
        let mut fetched_accs = rpc
            .get_multiple_accounts(&[pool_state_addr, mint.mint()])
            .await
            .unwrap();
        let lst_mint_acc = fetched_accs.pop().unwrap().unwrap();
        let pool_state_acc = fetched_accs.pop().unwrap().unwrap();

        let pool_state = try_pool_state(&pool_state_acc.data()).unwrap();
        verify_admin(pool_state, admin.pubkey()).unwrap();

        let (keys, _bumps) = AddLstFreeArgs {
            payer: payer.pubkey(),
            sol_value_calculator: sol_val_calc,
            pool_state: pool_state_acc,
            lst_mint: Keyed {
                pubkey: mint.mint(),
                account: lst_mint_acc,
            },
        }
        .resolve_for_prog(program_id)
        .unwrap();
        let ix = add_lst_ix_with_program_id(program_id, keys).unwrap();

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
