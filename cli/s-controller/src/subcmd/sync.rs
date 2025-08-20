use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_cli_utils::handle_tx_full;
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, sync_sol_value_ix_full_for_prog,
    SyncSolValueByMintFreeArgs, SyncSolValuePdas,
};
use sanctum_solana_client_utils::to_est_cu_sim_tx;
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
use std::str::FromStr;

use crate::{lst_arg::LstArg, rpc::does_tx_modify_pool_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Sync SOL value for a LST in the pool",
    long_about = "Sync SOL value for a LST in the pool.
To sync all LSTs in the pool that are on sanctum-lst-list, use sync-all."
)]
pub struct SyncArgs {
    #[arg(
        help = "Mint of the LST to sync SOL value for. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'"
    )]
    pub mint: String,

    #[arg(
        long,
        short,
        help = "Account suffix slice to call LstToSol for the given LST, excluding the SOL value calculator program ID and mint. Required if mint is not on sanctum-lst-list. Ignore clap's help msg and put this after mint arg instead of before.",
        value_delimiter = ' ',
        num_args = 1..,
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub suffix: Vec<Pubkey>,

    #[arg(
        long,
        short,
        help = "If flag set, will process tx even if pool does not need to sync this LST. Otherwise, simulates sync tx first and does nothing if no changes.",
        default_value_t = false
    )]
    pub force: bool,
}

impl SyncArgs {
    pub async fn run(args: crate::Args) {
        let slsts = args.load_slst_list();
        let Self {
            mint,
            suffix,
            force,
        } = match args.subcmd {
            Subcmd::Sync(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;
        let mint = LstArg::parse_arg(&mint, &slsts).unwrap();

        // accounts suffix slice including lst_mint as first account
        let suffix = mint.sol_value_calculator_accounts_of().unwrap_or_else(|| {
            std::iter::once(AccountMeta {
                pubkey: mint.mint(),
                is_signer: false,
                is_writable: false,
            })
            .chain(suffix.into_iter().map(|pk| AccountMeta {
                pubkey: pk,
                is_signer: false,
                is_writable: false,
            }))
            .collect()
        });

        let pool_state_addr = find_pool_state_address(program_id).0;
        let lst_state_list_addr = find_lst_state_list_address(program_id).0;
        let mint_addr = mint.mint();
        let mut fetched_accs = rpc
            .get_multiple_accounts(&[mint_addr, lst_state_list_addr, pool_state_addr])
            .await
            .unwrap();
        let pool_state_acc = fetched_accs.pop().unwrap().unwrap();
        let lst_state_list_acc = fetched_accs.pop().unwrap().unwrap();
        let mint_acc = fetched_accs.pop().unwrap().unwrap();

        let (keys, lst_index, sol_value_calculator_program_id) = SyncSolValueByMintFreeArgs {
            lst_state_list: lst_state_list_acc,
            lst_mint: Keyed {
                pubkey: mint_addr,
                account: &mint_acc,
            },
        }
        .resolve_with_pdas(SyncSolValuePdas {
            pool_state: pool_state_addr,
            lst_state_list: lst_state_list_addr,
        })
        .unwrap();
        let ixs = vec![sync_sol_value_ix_full_for_prog(
            program_id,
            keys,
            lst_index,
            &suffix,
            sol_value_calculator_program_id,
        )
        .unwrap()];

        if !force {
            let should_run = does_tx_modify_pool_state(
                &rpc,
                &to_est_cu_sim_tx(&payer.pubkey(), &ixs, &[]).unwrap(),
                Keyed {
                    pubkey: pool_state_addr,
                    account: &pool_state_acc,
                },
            )
            .await;
            if !should_run {
                eprintln!("Sync not required, --force not provided. Exiting.");
                return;
            }
        }

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            ixs,
            &[],
            &mut [payer.as_ref()],
        )
        .await;
    }
}
