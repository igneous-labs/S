use std::collections::HashMap;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use inquire::Confirm;
use s_controller_lib::{
    find_lst_state_list_address, find_pool_reserves_address, find_pool_state_address,
    FindLstPdaAtaKeys,
};
use s_jup_interface::{LstData, SPool, SPoolInitAccounts};
use s_sol_val_calc_prog_aggregate::LstSolValCalc;
use sanctum_solana_cli_utils::parse_signer;
use sanctum_token_lib::token_account_balance;
use solana_sdk::{account::Account, native_token::lamports_to_sol, pubkey::Pubkey};
use spl_token::native_mint;

use crate::{common::SANCTUM_LST_LIST, lst_amt_arg::LstAmtArg, lst_arg::LstArg};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Rebalance from SOL to another LST by staking the SOL to the LST's stake pool.",
    long_about = "Rebalance from SOL to another LST by staking the SOL to the LST's stake pool.
May require the payer to subsidize some amount of LST to make up for the stake pool's SOL deposit fees"
)]
pub struct RebalIntoSolArgs {
    #[arg(
        long,
        short,
        help = "The program's rebalance authority. Defaults to config wallet if not set."
    )]
    pub rebalance_auth: Option<String>,

    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Auto-confirm transaction"
    )]
    pub yes: bool,

    #[arg(
        help = "The LST to rebalance from into SOL",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub lst: LstArg,

    #[arg(
        help = "Amount in LST to rebalance",
        value_parser = StringValueParser::new().try_map(|s| LstAmtArg::parse_arg(&s)),
    )]
    pub lst_amt: LstAmtArg,
}

impl RebalIntoSolArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            rebalance_auth,
            yes,
            lst,
            lst_amt,
        } = match args.subcmd {
            Subcmd::RebalIntoSol(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let sanctum_lst = match lst {
            LstArg::SanctumLst(s) => s,
            LstArg::Unknown(lst) => {
                panic!("Unknown LST {lst}. Only LSTs on sanctum-lst-list supported")
            }
        };
        let symbol = &sanctum_lst.symbol;
        let (pool_id, _) = find_pool_state_address(program_id);
        let (lst_state_list_id, _) = find_lst_state_list_address(program_id);

        let rebalance_auth = rebalance_auth.map(|s| parse_signer(&s).unwrap());
        let rebalance_auth = rebalance_auth
            .as_ref()
            .map_or_else(|| payer.as_ref(), |s| s.as_ref());

        let amt = match lst_amt {
            LstAmtArg::Amt(v) => v,
            LstAmtArg::All => {
                let (wsol_reserves, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
                    lst_mint: sanctum_lst.mint,
                    token_program: spl_token::ID,
                });
                let fetched_reserves = rpc.get_account(&wsol_reserves).await.unwrap();
                token_account_balance(fetched_reserves).unwrap()
            }
        };

        let mut fetched = rpc
            .get_multiple_accounts(&[pool_id, lst_state_list_id])
            .await
            .unwrap();
        let lst_state_list_acc = fetched.pop().unwrap().unwrap();
        let pool_acc = fetched.pop().unwrap().unwrap();

        let mut spool = SPool::from_init_accounts(
            program_id,
            SPoolInitAccounts {
                lst_state_list: lst_state_list_acc,
                pool_state: pool_acc,
            },
            &SANCTUM_LST_LIST.sanctum_lst_list,
        )
        .unwrap();

        let mut accounts_to_fetch = spool.get_accounts_to_update_lsts_filtered(|state, _data| {
            state.mint == sanctum_lst.mint || state.mint == native_mint::ID
        });
        accounts_to_fetch.sort();
        accounts_to_fetch.dedup();

        // TODO: make sure accounts_to_fetch.len() < 5 or we get kicked by rpc
        let account_map: HashMap<Pubkey, Account> = rpc
            .get_multiple_accounts(&accounts_to_fetch)
            .await
            .unwrap()
            .into_iter()
            .zip(accounts_to_fetch)
            .filter_map(|(acc, pk)| acc.map(|acc| (pk, acc)))
            .collect();

        spool.update_full(&account_map).unwrap();

        let (_state, LstData { sol_val_calc, .. }) =
            spool.find_ready_lst(sanctum_lst.mint).unwrap();
        let required_lamports_deposit = sol_val_calc.lst_to_sol(amt).unwrap().get_max();
        let required_sol_deposit = lamports_to_sol(required_lamports_deposit);

        if yes {
            eprintln!("Depositing {required_sol_deposit} SOL")
        } else {
            let has_confirmed = Confirm::new(&format!(
                "Will need to deposit {required_sol_deposit} SOL. Proceed?",
            ))
            .with_default(false)
            .prompt()
            .unwrap();
            if !has_confirmed {
                return;
            }
        }
    }
}
