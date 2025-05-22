use std::sync::{atomic::AtomicU64, Arc};

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
    clock::Clock, native_token::lamports_to_sol, pubkey::Pubkey, system_instruction, sysvar,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};
use spl_token::{instruction::sync_native, native_mint};
use wsol_calculator_lib::WSOL_LST_SOL_COMMON_METAS;

use crate::{
    common::{fetch_srlut, SANCTUM_LST_LIST},
    lst_amt_arg::LstAmtArg,
    lst_arg::LstArg,
    rpc::fetch_accounts_as_map,
    stakedex_reimpl::WithdrawSolStakedex,
};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Rebalance from spl stake pool LST to SOL by withdrawing inactive SOL from the pool's reserves.",
    long_about = "Rebalance from spl stake pool LST to SOL by withdrawing inactive SOL from the pool's reserves.
May require the payer to subsidize some amount of LST to make up for the stake pool's SOL withdrawal fees."
)]
pub struct RebalWithdrawSolArgs {
    #[arg(
        long,
        short,
        help = "The program's rebalance authority. Defaults to config wallet if not set."
    )]
    pub rebalance_auth: Option<String>,

    #[arg(
        long,
        short,
        help = "System account to transfer SOL from to pay subsidies if required. Defaults to config wallet if not set."
    )]
    pub subsidy_payer: Option<String>,

    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Auto-confirm subsidy amount"
    )]
    pub yes: bool,

    #[arg(
        help = "Amount in LST to rebalance",
        value_parser = StringValueParser::new().try_map(|s| LstAmtArg::parse_arg(&s)),
    )]
    pub amt: LstAmtArg,

    #[arg(
        help = "The LST to rebalance SOL into",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub lst: LstArg,
}

impl RebalWithdrawSolArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            rebalance_auth,
            subsidy_payer,
            yes,
            amt,
            lst,
        } = match args.subcmd {
            Subcmd::RebalWithdrawSol(a) => a,
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

        let [rebalance_auth, subsidy_payer] = [rebalance_auth, subsidy_payer]
            .map(|opt| opt.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap())));
        let [rebalance_auth, subsidy_payer] = [&rebalance_auth, &subsidy_payer]
            .map(|opt| opt.as_ref().map_or_else(|| payer.as_ref(), |s| s.as_ref()));

        let amt = match amt {
            LstAmtArg::Amt(v) => v,
            LstAmtArg::All => {
                let (lst_reserves, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
                    lst_mint: sanctum_lst.mint,
                    token_program: sanctum_lst.token_program,
                });
                let fetched_reserves = rpc.get_account(&lst_reserves).await.unwrap();
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
        let mut withdraw_sol = WithdrawSolStakedex::from_sanctum_lst(sanctum_lst);

        let expected_rebalance_auth = try_pool_state(&spool.pool_state_data().unwrap())
            .unwrap()
            .rebalance_authority;
        if rebalance_auth.pubkey() != expected_rebalance_auth {
            panic!(
                "Wrong rebalance auth. Expecting {expected_rebalance_auth}, got {}",
                rebalance_auth.pubkey()
            )
        }

        let mut accounts_to_fetch = spool.get_accounts_to_update_lsts_filtered(|state, _data| {
            state.mint == sanctum_lst.mint || state.mint == native_mint::ID
        });
        accounts_to_fetch.sort();
        accounts_to_fetch.dedup();

        let account_map = fetch_accounts_as_map(&rpc, &accounts_to_fetch).await;

        spool.update_full(&account_map).unwrap();
        let _spl_reserves_missing_ignore_err: Result<_, _> = withdraw_sol.update(&account_map);
        // Spl needs to be updated twice before it can be used
        let account_map = fetch_accounts_as_map(&rpc, &withdraw_sol.get_accounts_to_update()).await;
        withdraw_sol.update(&account_map).unwrap();

        let (
            _state,
            LstData {
                sol_val_calc,
                reserves_balance,
                ..
            },
        ) = spool.find_ready_lst(sanctum_lst.mint).unwrap();
        let required_sol_deposit = {
            let init_reserves = reserves_balance.unwrap();
            let init_sol_val = sol_val_calc.lst_to_sol(init_reserves).unwrap().get_min();
            let aft_start_rebal_sol_val = sol_val_calc
                .lst_to_sol(init_reserves - amt)
                .unwrap()
                .get_min();
            init_sol_val - aft_start_rebal_sol_val
        };

        let sol_withdrawn = withdraw_sol.quote_withdraw_sol(amt).unwrap();

        let subsidy_amt = required_sol_deposit.saturating_sub(sol_withdrawn);
        if subsidy_amt > 0 {
            let subsidy_amt_decimals = lamports_to_sol(subsidy_amt);
            if yes {
                eprintln!("Subsidizing {subsidy_amt_decimals} SOL")
            } else {
                let has_confirmed = Confirm::new(&format!(
                    "Will need to subsidize {subsidy_amt_decimals} SOL. Proceed?",
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

        // 2 CB ixs + create ata + start + withdraw sol + transfer subsidy + syncnative + end
        let mut ixs = Vec::with_capacity(8);

        let lst_withdraw_to = get_associated_token_address_with_program_id(
            &payer.pubkey(),
            &sanctum_lst.mint,
            &sanctum_lst.token_program,
        );

        let mut fetched = rpc.get_multiple_accounts(&[lst_withdraw_to]).await.unwrap();
        let wsol_subsidize_from_acc = fetched.pop().unwrap();
        if subsidy_amt > 0 {
            match wsol_subsidize_from_acc {
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
        let lst_withdraw_to_acc = fetched.pop().unwrap();
        if lst_withdraw_to_acc.is_none() {
            ixs.push(create_associated_token_account_idempotent(
                &payer.pubkey(),
                &payer.pubkey(),
                &sanctum_lst.mint,
                &sanctum_lst.token_program,
            ));
        }

        let start_rebalance_ix = start_rebalance_ix_by_mints_full_for_prog(
            program_id,
            StartRebalanceByMintsFreeArgs {
                withdraw_to: lst_withdraw_to,
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
        ixs.push(start_rebalance_ix);
        let (wsol_reserves, _) = find_pool_reserves_address_with_pool_state_id(
            pool_id,
            FindLstPdaAtaKeys {
                lst_mint: native_mint::ID,
                token_program: spl_token::ID,
            },
        );
        ixs.push(
            withdraw_sol
                .withdraw_sol_ix(&SwapParams {
                    in_amount: amt,
                    out_amount: sol_withdrawn,
                    source_mint: sanctum_lst.mint,
                    destination_mint: native_mint::ID,
                    source_token_account: lst_withdraw_to,
                    destination_token_account: wsol_reserves,
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
            ixs.push(system_instruction::transfer(
                &subsidy_payer.pubkey(),
                &wsol_reserves,
                subsidy_amt,
            ))
        }
        ixs.push(sync_native(&spl_token::ID, &wsol_reserves).unwrap());
        ixs.push(end_rebalance_ix);

        let srlut = fetch_srlut(&rpc, &args.lut).await;

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
