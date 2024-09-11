use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_controller_lib::{
    end_rebalance_ix_from_start_rebalance_ix, find_lst_state_list_address, find_pool_state_address,
    start_rebalance_ix_by_mints_full_for_prog, SrcDstLstSolValueCalcAccountSuffixes,
    StartRebalanceByMintsFreeArgs, StartRebalanceIxLstAmts,
};
use sanctum_solana_cli_utils::PubkeySrc;
use sanctum_token_lib::MintWithTokenProgram;
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{native_token::sol_to_lamports, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::lst_arg::LstArg;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Output the StartRebalance and EndRebalance instructions as base58-encoded transactions",
    long_about = "Output the StartRebalance and EndRebalance instructions as base58-encoded transactions.
Mainly for use with squads UI"
)]
pub struct RebalB58Args {
    #[arg(
        long,
        short,
        help = "The src LST token account to withdraw the src LST to. Defaults to config wallet's ATA if not set"
    )]
    pub withdraw_to: Option<String>,

    #[arg(help = "Amount of src LST to rebalance")]
    pub src_amt: f64,

    #[arg(
        help = "The LST to rebalance from",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub src_lst: LstArg,

    #[arg(
        help = "The LST to rebalance to",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub dst_lst: LstArg,
}

impl RebalB58Args {
    pub async fn run(args: crate::Args) {
        let Self {
            withdraw_to,
            src_amt,
            src_lst,
            dst_lst,
        } = match args.subcmd {
            Subcmd::RebalB58(a) => a,
            _ => unreachable!(),
        };

        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let [src_sanctum_lst, dst_sanctum_lst] = [src_lst, dst_lst].map(|lst| match lst {
            LstArg::SanctumLst(s) => s,
            LstArg::Unknown(lst) => {
                panic!("Unknown LST {lst}. Only LSTs on sanctum-lst-list supported")
            }
        });

        let withdraw_to = withdraw_to.map_or_else(
            || {
                get_associated_token_address_with_program_id(
                    &args.config.signer().pubkey(),
                    &src_sanctum_lst.mint,
                    &src_sanctum_lst.token_program,
                )
            },
            |w| PubkeySrc::parse(&w).unwrap().pubkey(),
        );

        let (pool_id, _) = find_pool_state_address(program_id);
        let (lst_state_list_id, _) = find_lst_state_list_address(program_id);

        let mut fetched = rpc
            .get_multiple_accounts(&[pool_id, lst_state_list_id])
            .await
            .unwrap();
        let lst_state_list_acc = fetched.pop().unwrap().unwrap();
        let pool_acc = fetched.pop().unwrap().unwrap();

        let start_rebalance_ix = start_rebalance_ix_by_mints_full_for_prog(
            program_id,
            StartRebalanceByMintsFreeArgs {
                withdraw_to,
                lst_state_list: Keyed {
                    pubkey: lst_state_list_id,
                    account: &lst_state_list_acc,
                },
                pool_state: Keyed {
                    pubkey: pool_id,
                    account: &pool_acc,
                },
                src_lst_mint: MintWithTokenProgram {
                    pubkey: src_sanctum_lst.mint,
                    token_program: src_sanctum_lst.token_program,
                },
                dst_lst_mint: MintWithTokenProgram {
                    pubkey: dst_sanctum_lst.mint,
                    token_program: dst_sanctum_lst.token_program,
                },
            },
            StartRebalanceIxLstAmts {
                amount: sol_to_lamports(src_amt),
                // TODO: allow for slippage config
                min_starting_src_lst: 0,
                max_starting_dst_lst: u64::MAX,
            },
            SrcDstLstSolValueCalcAccountSuffixes {
                src_lst_calculator_accounts: &src_lst.sol_value_calculator_accounts_of().unwrap(),
                dst_lst_calculator_accounts: &dst_lst.sol_value_calculator_accounts_of().unwrap(),
            },
        )
        .unwrap();
        let end_rebalance_ix =
            end_rebalance_ix_from_start_rebalance_ix(&start_rebalance_ix).unwrap();

        for ix in [start_rebalance_ix, end_rebalance_ix] {
            let tx = Transaction::new_with_payer(&[ix], None);
            let bytes = tx.message.serialize();
            println!("{}", bs58::encode(bytes).into_string());
        }
    }
}
