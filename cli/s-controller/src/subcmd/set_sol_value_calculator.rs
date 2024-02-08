use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address,
    set_sol_value_calculator_ix_by_mint_full_with_program_id, try_pool_state,
    SetSolValueCalculatorByMintFreeArgs,
};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    instruction::AccountMeta,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};

use crate::{common::verify_admin, lst_arg::LstArg};

use super::Subcmd;

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
        help = "The LST's SOL value calculator program to set to.",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub sol_val_calc: Pubkey,

    #[arg(
        long,
        short,
        help = "Mint of the LST to set SOL value calculator program for. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub mint: LstArg,

    #[arg(
        long = "account-suffix",
        short = 'c',
        help = "List of pubkeys for account suffix for the SOL value calculator program.",
        num_args(1..),
    )]
    pub account_suffix: Vec<Pubkey>,
}

impl SetSolValueCalculatorArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            admin,
            sol_val_calc,
            mint,
            account_suffix,
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
        let lst_mint_acc = fetched.pop().unwrap().unwrap();
        let lst_state_list_acc = fetched.pop().unwrap().unwrap();
        let pool_state_acc = fetched.pop().unwrap().unwrap();

        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        verify_admin(pool_state, admin.pubkey()).unwrap();

        let sol_value_calculator_accounts: Vec<AccountMeta> = account_suffix
            .into_iter()
            .map(|pubkey| AccountMeta {
                pubkey,
                is_signer: false,
                is_writable: false,
            })
            .collect();
        let ix = set_sol_value_calculator_ix_by_mint_full_with_program_id(
            program_id,
            &SetSolValueCalculatorByMintFreeArgs {
                pool_state: pool_state_acc,
                lst_state_list: lst_state_list_acc,
                lst_mint: KeyedAccount {
                    pubkey: mint.mint(),
                    account: lst_mint_acc,
                },
            },
            &sol_value_calculator_accounts,
            sol_val_calc,
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
