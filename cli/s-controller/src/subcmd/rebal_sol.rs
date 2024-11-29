use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc},
};

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use inquire::Confirm;
use jupiter_amm_interface::SwapParams;
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use s_controller_lib::{
    end_rebalance_ix_from_start_rebalance_ix, find_lst_state_list_address,
    find_pool_reserves_address, find_pool_reserves_address_with_pool_state_id,
    find_pool_state_address, start_rebalance_ix_by_mints_full_for_prog, try_pool_state,
    FindLstPdaAtaKeys, SrcDstLstSolValueCalcAccountSuffixes, StartRebalanceByMintsFreeArgs,
    StartRebalanceIxLstAmts,
};
use s_jup_interface::{LstData, SPool, SPoolInitAccounts};
use s_sol_val_calc_prog_aggregate::LstSolValCalc;
use sanctum_solana_cli_utils::PubkeySrc;
use sanctum_token_lib::{token_account_balance, MintWithTokenProgram};
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{
    account::Account, clock::Clock, native_token::lamports_to_sol, pubkey::Pubkey, sysvar,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};
use spl_token::native_mint;
use wsol_calculator_lib::WSOL_LST_SOL_COMMON_METAS;

use crate::{
    common::{fetch_srlut, SANCTUM_LST_LIST},
    lst_amt_arg::LstAmtArg,
    lst_arg::LstArg,
    stakedex_reimpl::DepositSolStakedex,
};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Rebalance from SOL to another LST by staking the SOL to the LST's stake pool.",
    long_about = "Rebalance from SOL to another LST by staking the SOL to the LST's stake pool.
May require the payer to subsidize some amount of LST to make up for the stake pool's SOL deposit fees"
)]
pub struct RebalSolArgs {
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
        help = "Auto-confirm subsidy amount"
    )]
    pub yes: bool,

    #[arg(
        help = "Amount in SOL to rebalance",
        value_parser = StringValueParser::new().try_map(|s| LstAmtArg::parse_arg(&s)),
    )]
    pub sol: LstAmtArg,

    #[arg(
        help = "The LST to rebalance SOL into",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub lst: LstArg,
}

impl RebalSolArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            rebalance_auth,
            yes,
            sol,
            lst,
        } = match args.subcmd {
            Subcmd::RebalSol(a) => a,
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

        let rebalance_auth =
            rebalance_auth.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let rebalance_auth = rebalance_auth
            .as_ref()
            .map_or_else(|| payer.as_ref(), |s| s.as_ref());

        let lamports = match sol {
            LstAmtArg::Amt(v) => v,
            LstAmtArg::All => {
                let (wsol_reserves, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
                    lst_mint: native_mint::ID,
                    token_program: spl_token::ID,
                });
                let fetched_reserves = rpc.get_account(&wsol_reserves).await.unwrap();
                token_account_balance(fetched_reserves).unwrap()
            }
        };

        let mut fetched = rpc
            .get_multiple_accounts(&[pool_id, lst_state_list_id, sysvar::clock::ID])
            .await
            .unwrap();
        let clock = fetched.pop().unwrap().unwrap();
        let clock: Clock = bincode::deserialize(&clock.data).unwrap();
        let lst_state_list_acc = fetched.pop().unwrap().unwrap();
        let pool_acc = fetched.pop().unwrap().unwrap();

        let mut spool = SPool::from_init_accounts(
            program_id,
            SPoolInitAccounts {
                lst_state_list: lst_state_list_acc,
                pool_state: pool_acc,
            },
            &SANCTUM_LST_LIST.sanctum_lst_list,
            &Arc::new(AtomicU64::new(clock.epoch)),
        )
        .unwrap();
        let mut deposit_sol = DepositSolStakedex::from_sanctum_lst(sanctum_lst);

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
        deposit_sol.update(&account_map).unwrap();

        let (_state, LstData { sol_val_calc, .. }) =
            spool.find_ready_lst(sanctum_lst.mint).unwrap();
        let required_lst_deposit = sol_val_calc.sol_to_lst(lamports).unwrap().get_max();

        let lst_minted = deposit_sol.quote_deposit_sol(lamports).unwrap();

        let subsidy_amt = required_lst_deposit.saturating_sub(lst_minted);

        if subsidy_amt > 0 {
            let subsidy_amt_decimals = lamports_to_sol(subsidy_amt);
            if yes {
                eprintln!("Subsidizing {subsidy_amt_decimals} {symbol}")
            } else {
                let has_confirmed = Confirm::new(&format!(
                    "Will need to subsidize {subsidy_amt_decimals} {symbol}. Proceed?",
                ))
                .with_default(false)
                .prompt()
                .unwrap();
                if !has_confirmed {
                    return;
                }
            }
        } else {
            eprintln!("No subsidy required, proceeding");
        }

        let expected_rebalance_auth = try_pool_state(&spool.pool_state_data().unwrap())
            .unwrap()
            .rebalance_authority;
        if rebalance_auth.pubkey() != expected_rebalance_auth {
            panic!(
                "Wrong rebalance auth. Expecting {expected_rebalance_auth}, got {}",
                rebalance_auth.pubkey()
            )
        }

        // 2 CB ixs + create wsol ata + start + depositSol + transfer + end
        let mut ixs = Vec::with_capacity(7);

        let [wsol_withdraw_to, lst_subsidize_from] = [
            (native_mint::ID, spl_token::ID),
            (sanctum_lst.mint, sanctum_lst.token_program),
        ]
        .map(|(mint, token_program)| {
            get_associated_token_address_with_program_id(&payer.pubkey(), &mint, &token_program)
        });

        let mut fetched = rpc
            .get_multiple_accounts(&[wsol_withdraw_to, lst_subsidize_from])
            .await
            .unwrap();
        let lst_subsidize_from_acc = fetched.pop().unwrap();
        if subsidy_amt > 0 {
            match lst_subsidize_from_acc {
                Some(a) => {
                    let ata_balance = token_account_balance(a).unwrap();
                    if ata_balance < subsidy_amt {
                        panic!("Expected payer {symbol} ATA to have at least {subsidy_amt} for subsidy, but it only has {ata_balance}");
                    }
                }
                None => {
                    panic!("Expected payer to have {symbol} ATA with at least {subsidy_amt} balance for subsidy");
                }
            }
        }
        let wsol_withdraw_to_acc = fetched.pop().unwrap();
        if wsol_withdraw_to_acc.is_none() {
            ixs.push(create_associated_token_account_idempotent(
                &payer.pubkey(),
                &payer.pubkey(),
                &native_mint::ID,
                &spl_token::ID,
            ));
        }

        let start_rebalance_ix = start_rebalance_ix_by_mints_full_for_prog(
            program_id,
            StartRebalanceByMintsFreeArgs {
                withdraw_to: wsol_withdraw_to,
                lst_state_list: Keyed {
                    pubkey: lst_state_list_id,
                    account: &spool.lst_state_list_account,
                },
                pool_state: Keyed {
                    pubkey: pool_id,
                    account: &spool.pool_state_account.unwrap(),
                },
                src_lst_mint: MintWithTokenProgram {
                    pubkey: native_mint::ID,
                    token_program: spl_token::ID,
                },
                dst_lst_mint: MintWithTokenProgram {
                    pubkey: sanctum_lst.mint,
                    token_program: sanctum_lst.token_program,
                },
            },
            StartRebalanceIxLstAmts {
                amount: lamports,
                // TODO: allow for slippage config
                min_starting_src_lst: 0,
                max_starting_dst_lst: u64::MAX,
            },
            SrcDstLstSolValueCalcAccountSuffixes {
                src_lst_calculator_accounts: &WSOL_LST_SOL_COMMON_METAS,
                dst_lst_calculator_accounts: &lst.sol_value_calculator_accounts_of().unwrap(),
            },
        )
        .unwrap();
        let end_rebalance_ix =
            end_rebalance_ix_from_start_rebalance_ix(&start_rebalance_ix).unwrap();
        ixs.push(start_rebalance_ix);
        let (lst_reserves, _) = find_pool_reserves_address_with_pool_state_id(
            pool_id,
            FindLstPdaAtaKeys {
                lst_mint: sanctum_lst.mint,
                token_program: sanctum_lst.token_program,
            },
        );
        ixs.push(
            deposit_sol
                .deposit_sol_ix(&SwapParams {
                    in_amount: lamports,
                    out_amount: lst_minted,
                    source_mint: native_mint::ID,
                    destination_mint: sanctum_lst.mint,
                    source_token_account: wsol_withdraw_to,
                    destination_token_account: lst_reserves,
                    token_transfer_authority: payer.pubkey(),
                    // dont cares
                    open_order_address: None,
                    quote_mint_to_referrer: None,
                    jupiter_program_id: &Pubkey::default(),
                    missing_dynamic_accounts_as_default: false,
                })
                .unwrap(),
        );
        if subsidy_amt > 0 {
            ixs.push(
                spl_token::instruction::transfer(
                    &sanctum_lst.token_program,
                    &lst_subsidize_from,
                    &lst_reserves,
                    &payer.pubkey(),
                    &[],
                    subsidy_amt,
                )
                .unwrap(),
            )
        }
        ixs.push(end_rebalance_ix);

        let srlut = fetch_srlut(&rpc).await;

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            ixs,
            &[srlut],
            &mut [payer.as_ref(), rebalance_auth],
        )
        .await;
    }
}
