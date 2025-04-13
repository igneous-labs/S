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
    find_pool_reserves_address, find_pool_state_address, start_rebalance_ix_by_mints_full_for_prog,
    try_pool_state, FindLstPdaAtaKeys, SrcDstLstSolValueCalcAccountSuffixes,
    StartRebalanceByMintsFreeArgs, StartRebalanceIxLstAmts,
};
use s_jup_interface::{apply_sync_sol_value, SPool, SPoolInitAccounts};
use s_sol_val_calc_prog_aggregate::LstSolValCalc;
use sanctum_solana_cli_utils::PubkeySrc;
use sanctum_token_lib::{token_account_balance, MintWithTokenProgram};
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{clock::Clock, native_token::lamports_to_sol, system_instruction, sysvar};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};
use stakedex_sdk_common::{DepositStakeInfo, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS};

use crate::{
    common::{fetch_srlut, sol_value_calculator_accounts_of_sanctum_lst, SANCTUM_LST_LIST},
    lst_amt_arg::LstAmtArg,
    lst_arg::LstArg,
    rpc::{fetch_accounts_as_map, find_unused_stake_prog_create_with_seed},
    stakedex_reimpl::{
        first_avail_withdraw_deposit_stake_quote, DepositStakeStakedex, WithdrawStakeInfo,
        WithdrawStakeStakedex,
    },
};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Rebalance from one LST to another by withdrawing stake from one and depositing it into the other.",
    long_about = "Rebalance from one LST to another by withdrawing stake from one and depositing it into the other.
May require the payer to subsidize some amount of LST to make up for the stake pools' fees.
Note that there is also an implicit subsidy of `STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS` SOL for every call to this subcmd,
which is used to prefund the blank stake account to withdraw stake to"
)]
pub struct RebalStakeArgs {
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
        help = "The LST to rebalance and withdraw stake from",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub from: LstArg,

    #[arg(
        help = "The LST to rebalance and deposit stake into",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub to: LstArg,

    #[arg(
        help = "Amount in `from` LST to rebalance",
        value_parser = StringValueParser::new().try_map(|s| LstAmtArg::parse_arg(&s)),
    )]
    pub amt: LstAmtArg,
}

impl RebalStakeArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            rebalance_auth,
            yes,
            from,
            to,
            amt,
        } = match args.subcmd {
            Subcmd::RebalStake(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let [from, to] = [from, to].map(|lst| match lst {
            LstArg::SanctumLst(s) => s,
            LstArg::Unknown(lst) => {
                panic!("Unknown LST {lst}. Only LSTs on sanctum-lst-list supported")
            }
        });
        let to_symbol = to.symbol.as_str();

        let (pool_id, _) = find_pool_state_address(program_id);
        let (lst_state_list_id, _) = find_lst_state_list_address(program_id);

        let rebalance_auth =
            rebalance_auth.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let rebalance_auth = rebalance_auth
            .as_ref()
            .map_or_else(|| payer.as_ref(), |s| s.as_ref());

        let amt = match amt {
            LstAmtArg::Amt(v) => v,
            LstAmtArg::All => {
                let (from_reserves, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
                    lst_mint: from.mint,
                    token_program: from.token_program,
                });
                let fetched_reserves = rpc.get_account(&from_reserves).await.unwrap();
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
        let mut withdraw_stake = WithdrawStakeStakedex::from_sanctum_lst(from);
        let mut deposit_stake = DepositStakeStakedex::from_sanctum_lst(to);

        let expected_rebalance_auth = try_pool_state(&spool.pool_state_data().unwrap())
            .unwrap()
            .rebalance_authority;
        let actual_rebalance_auth = rebalance_auth.pubkey();
        if actual_rebalance_auth != expected_rebalance_auth {
            panic!(
                "Wrong rebalance auth. Expecting {expected_rebalance_auth}, got {actual_rebalance_auth}",
            )
        }

        let mut accounts_to_fetch = [
            spool.get_accounts_to_update_lsts_filtered(|state, _data| {
                state.mint == from.mint || state.mint == to.mint
            }),
            withdraw_stake.get_accounts_to_update(),
            deposit_stake.get_accounts_to_update(),
        ]
        .concat();
        accounts_to_fetch.sort();
        accounts_to_fetch.dedup();

        let account_map = fetch_accounts_as_map(&rpc, &accounts_to_fetch).await;

        spool.update_full(&account_map).unwrap();
        withdraw_stake.update(&account_map).unwrap();
        deposit_stake.update(&account_map).unwrap();

        let (wsq, dsq) =
            first_avail_withdraw_deposit_stake_quote(amt, &withdraw_stake, &deposit_stake).unwrap();

        let pool_state = *try_pool_state(&spool.pool_state_data().unwrap()).unwrap();
        let subsidy_amt = {
            let [(from_lst_state, from_lst_data), (to_lst_state, to_lst_data)] =
                [from, to].map(|slst| spool.find_ready_lst(slst.mint).unwrap());
            let (pool_state, from_lst_state, from_reserves_balance) =
                apply_sync_sol_value(pool_state, from_lst_state, from_lst_data).unwrap();
            let (_pool_state, to_lst_state, to_reserves_balance) =
                apply_sync_sol_value(pool_state, to_lst_state, to_lst_data).unwrap();

            // SyncSolValue counts by lst_to_sol().get_min().
            // Determine how much SOL value the pool will drop by from StartRebalance by taking
            // old_sol_value - new_sol_value
            // = old_sol_value - lst_to_sol(remainder).get_min()
            let from_sol_val = from_lst_state.sol_value.saturating_sub(
                from_lst_data
                    .sol_val_calc
                    .lst_to_sol(from_reserves_balance.saturating_sub(amt))
                    .unwrap()
                    .get_min(),
            );
            let required_lst_deposit = {
                let required_to_sol_val = to_lst_state.sol_value.saturating_add(from_sol_val);
                let mut ending_to_balance = to_lst_data
                    .sol_val_calc
                    .sol_to_lst(required_to_sol_val)
                    .unwrap()
                    .get_max();
                // account for rounding error
                while to_lst_data
                    .sol_val_calc
                    .lst_to_sol(ending_to_balance)
                    .unwrap()
                    .get_min()
                    < required_to_sol_val
                {
                    ending_to_balance += 1;
                }
                ending_to_balance.saturating_sub(to_reserves_balance)
            };
            required_lst_deposit.saturating_sub(dsq.tokens_out)
        };
        if subsidy_amt > 0 {
            let subsidy_amt_decimals = lamports_to_sol(subsidy_amt);
            if yes {
                eprintln!("Subsidizing {subsidy_amt_decimals} {to_symbol}")
            } else {
                let has_confirmed = Confirm::new(&format!(
                    "Will need to subsidize {subsidy_amt_decimals} {to_symbol}. Proceed?",
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

        let mut ixs = vec![];
        let [withdraw_to, subsidize_from] = [from, to].map(|lst| {
            get_associated_token_address_with_program_id(
                &payer.pubkey(),
                &lst.mint,
                &lst.token_program,
            )
        });
        let mut fetched = rpc
            .get_multiple_accounts(&[withdraw_to, subsidize_from])
            .await
            .unwrap();
        let subsidize_from_acc = fetched.pop().unwrap();
        if subsidy_amt > 0 {
            match subsidize_from_acc {
                Some(a) => {
                    let ata_balance = token_account_balance(a).unwrap();
                    if ata_balance < subsidy_amt {
                        panic!("Expected payer {to_symbol} ATA to have at least {subsidy_amt} for subsidy, but it only has {ata_balance}");
                    }
                }
                None => {
                    panic!("Expected payer to have {to_symbol} ATA with at least {subsidy_amt} balance for subsidy");
                }
            }
        }
        let withdraw_to_acc = fetched.pop().unwrap();
        if withdraw_to_acc.is_none() {
            ixs.push(create_associated_token_account_idempotent(
                &payer.pubkey(),
                &payer.pubkey(),
                &from.mint,
                &from.token_program,
            ));
        }

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
                    pubkey: from.mint,
                    token_program: from.token_program,
                },
                dst_lst_mint: MintWithTokenProgram {
                    pubkey: to.mint,
                    token_program: to.token_program,
                },
            },
            StartRebalanceIxLstAmts {
                amount: amt,
                // TODO: allow for slippage config
                min_starting_src_lst: 0,
                max_starting_dst_lst: u64::MAX,
            },
            SrcDstLstSolValueCalcAccountSuffixes {
                src_lst_calculator_accounts: &sol_value_calculator_accounts_of_sanctum_lst(from),
                dst_lst_calculator_accounts: &sol_value_calculator_accounts_of_sanctum_lst(to),
            },
        )
        .unwrap();
        let end_rebalance_ix =
            end_rebalance_ix_from_start_rebalance_ix(&start_rebalance_ix).unwrap();
        ixs.push(start_rebalance_ix);
        let (bridge_stake, bridge_stake_seed) =
            find_unused_stake_prog_create_with_seed(&rpc, &payer.pubkey()).await;

        // need to prefund bridge stake
        ixs.push(system_instruction::transfer(
            &payer.pubkey(),
            &bridge_stake,
            STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS,
        ));

        ixs.extend(
            withdraw_stake
                .withdraw_stake_ix(
                    &SwapParams {
                        in_amount: amt,
                        source_token_account: withdraw_to,
                        token_transfer_authority: payer.pubkey(),
                        // dont-cares
                        out_amount: Default::default(),
                        source_mint: Default::default(),
                        destination_mint: Default::default(),
                        destination_token_account: Default::default(),
                        open_order_address: Default::default(),
                        quote_mint_to_referrer: Default::default(),
                        jupiter_program_id: &Default::default(),
                        missing_dynamic_accounts_as_default: Default::default(),
                    },
                    &wsq,
                    &WithdrawStakeInfo {
                        seed: bridge_stake_seed,
                    },
                )
                .unwrap(),
        );
        let to_reserves = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint: to.mint,
            token_program: to.token_program,
        })
        .0;
        ixs.extend(
            deposit_stake
                .deposit_stake_ixs(
                    &SwapParams {
                        token_transfer_authority: payer.pubkey(),
                        destination_token_account: to_reserves,
                        // dont-cares
                        out_amount: Default::default(),
                        source_mint: Default::default(),
                        destination_mint: Default::default(),
                        open_order_address: Default::default(),
                        quote_mint_to_referrer: Default::default(),
                        jupiter_program_id: &Default::default(),
                        missing_dynamic_accounts_as_default: Default::default(),
                        in_amount: Default::default(),
                        source_token_account: Default::default(),
                    },
                    &dsq,
                    &DepositStakeInfo { addr: bridge_stake },
                )
                .unwrap(),
        );
        if subsidy_amt > 0 {
            ixs.push(
                spl_token::instruction::transfer_checked(
                    &to.token_program,
                    &subsidize_from,
                    &to.mint,
                    &to_reserves,
                    &payer.pubkey(),
                    &[],
                    subsidy_amt,
                    to.decimals,
                )
                .unwrap(),
            );
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
