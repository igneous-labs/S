use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_controller_interface::{
    set_sol_value_calculator_ix_with_program_id, SetSolValueCalculatorIxArgs,
};
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, index_to_u32, try_pool_state,
    SetSolValueCalculatorByMintFreeArgs,
};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};

use crate::{common::verify_admin, lst_arg::LstArg};

use super::Subcmd;

// TODO: accept cpi accounts for sync_sol_value (see processor)
#[derive(Args, Debug)]
#[command(long_about = "Sets the SOL value calculator program for a LST.")]
pub struct SetSolValueCalculatorArgs {
    #[arg(
        long,
        short,
        help = "The program's admin signer. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,

    #[arg(
        long,
        short,
        help = "The LST's SOL value calculator program.",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub sol_val_calc: Pubkey,

    #[arg(
        help = "Mint of the new LST to add. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub mint: LstArg,
}

impl SetSolValueCalculatorArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            admin,
            sol_val_calc: _sol_val_calc,
            mint,
        } = match args.subcmd {
            Subcmd::SetSolValueCalculator(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let admin_signer = admin.map(|s| parse_signer(&s).unwrap());
        let admin = admin_signer.as_ref().unwrap_or(&payer);

        let pool_state_addr = find_pool_state_address(program_id).0;
        let lst_state_list_addr = find_lst_state_list_address(program_id).0;
        let mut fetched = rpc
            .get_multiple_accounts(&[pool_state_addr, lst_state_list_addr, mint.mint()])
            .await
            .unwrap();
        let pool_state_acc = fetched.pop().unwrap().unwrap();
        let lst_state_list_acc = fetched.pop().unwrap().unwrap();
        let lst_mint_acc = fetched.pop().unwrap().unwrap();

        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        verify_admin(pool_state, admin.pubkey()).unwrap();

        let (keys, lst_index) = SetSolValueCalculatorByMintFreeArgs {
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
            lst_mint: KeyedAccount {
                pubkey: mint.mint(),
                account: lst_mint_acc,
            },
        }
        .resolve_for_prog(program_id)
        .unwrap();

        // TODO: replace ix with this with sol_val_calc and additional accounts for lst to sol call
        // set_sol_value_calculator_ix_by_mint_full_with_program_id(
        //     program_id,
        //     SetSolValueCalculatorIxArgs {
        //         lst_index: index_to_u32(lst_index).unwrap(),
        //     },
        //     &[],
        // );
        let ix = set_sol_value_calculator_ix_with_program_id(
            program_id,
            keys,
            SetSolValueCalculatorIxArgs {
                lst_index: index_to_u32(lst_index).unwrap(),
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
