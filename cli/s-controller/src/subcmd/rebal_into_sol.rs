use std::collections::HashMap;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use inquire::Confirm;
use s_cli_utils::handle_tx_full;
use s_controller_lib::{
    end_rebalance_ix_from_start_rebalance_ix, find_lst_state_list_address,
    find_pool_reserves_address, find_pool_state_address, start_rebalance_ix_by_mints_full_for_prog,
    FindLstPdaAtaKeys, SrcDstLstSolValueCalcAccountSuffixes, StartRebalanceByMintsFreeArgs,
    StartRebalanceIxLstAmts,
};
use s_jup_interface::{LstData, SPool, SPoolInitAccounts};
use s_sol_val_calc_prog_aggregate::LstSolValCalc;
use sanctum_solana_cli_utils::parse_signer;
use sanctum_token_lib::{token_account_balance, MintWithTokenProgram};
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{
    account::Account, native_token::lamports_to_sol, pubkey::Pubkey, system_instruction,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::native_mint;
use wsol_calculator_lib::WSOL_LST_SOL_COMMON_METAS;

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
        let (pool_id, _) = find_pool_state_address(program_id);
        let (lst_state_list_id, _) = find_lst_state_list_address(program_id);

        let rebalance_auth = rebalance_auth.map(|s| parse_signer(&s).unwrap());
        let rebalance_auth = rebalance_auth
            .as_ref()
            .map_or_else(|| payer.as_ref(), |s| s.as_ref());

        let amt = match lst_amt {
            LstAmtArg::Amt(v) => v,
            LstAmtArg::All => {
                let (src_lst_reserves, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
                    lst_mint: sanctum_lst.mint,
                    token_program: sanctum_lst.token_program,
                });
                let fetched_reserves = rpc.get_account(&src_lst_reserves).await.unwrap();
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
        // TODO: need to fix off by one
        let required_lamports_deposit = sol_val_calc.lst_to_sol(amt).unwrap().get_max() + 1;
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

        let withdraw_to = get_associated_token_address_with_program_id(
            &payer.pubkey(),
            &sanctum_lst.mint,
            &sanctum_lst.token_program,
        );
        let start_rebalance_ix = start_rebalance_ix_by_mints_full_for_prog(
            program_id,
            StartRebalanceByMintsFreeArgs {
                withdraw_to,
                lst_state_list: Keyed {
                    pubkey: lst_state_list_id,
                    account: &spool.lst_state_list_account,
                },
                pool_state: Keyed {
                    pubkey: pool_id,
                    account: &spool.pool_state_account.unwrap(),
                },
                src_lst_mint: MintWithTokenProgram {
                    pubkey: sanctum_lst.mint,
                    token_program: sanctum_lst.token_program,
                },
                dst_lst_mint: MintWithTokenProgram {
                    pubkey: native_mint::ID,
                    token_program: spl_token::ID,
                },
            },
            StartRebalanceIxLstAmts {
                amount: amt,
                // TODO: allow for slippage config
                min_starting_src_lst: 0,
                max_starting_dst_lst: u64::MAX,
            },
            SrcDstLstSolValueCalcAccountSuffixes {
                src_lst_calculator_accounts: &lst.sol_value_calculator_accounts_of().unwrap(),
                dst_lst_calculator_accounts: &WSOL_LST_SOL_COMMON_METAS,
            },
        )
        .unwrap();
        let end_rebalance_ix =
            end_rebalance_ix_from_start_rebalance_ix(&start_rebalance_ix).unwrap();
        let (wsol_reserves, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint: native_mint::ID,
            token_program: spl_token::ID,
        });

        let ixs = vec![
            start_rebalance_ix,
            system_instruction::transfer(
                &payer.pubkey(),
                &wsol_reserves,
                required_lamports_deposit,
            ),
            spl_token::instruction::sync_native(&spl_token::ID, &wsol_reserves).unwrap(),
            end_rebalance_ix,
        ];

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            ixs,
            &[],
            &mut [payer.as_ref(), rebalance_auth],
        )
        .await;
    }
}
