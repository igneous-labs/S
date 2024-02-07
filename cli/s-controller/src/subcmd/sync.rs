use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, sync_sol_value_ix_full_for_prog,
    try_find_lst_mint_on_list, try_lst_state_list, SyncSolValueByMintFreeArgs, SyncSolValuePdas,
};
use sanctum_solana_cli_utils::TxSendingNonblockingRpcClient;
use solana_readonly_account::keyed::Keyed;
use solana_sdk::{
    instruction::AccountMeta,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};
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
        help = "Mint of the LST to sync SOL value for. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub mint: LstArg,

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

        let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
        // TODO: optim - resolve_with_pdas() below calls try_find_lst_mint_on_list() again
        let sol_val_calc_program_id = try_find_lst_mint_on_list(mint_addr, lst_state_list)
            .unwrap()
            .1
            .sol_value_calculator;

        let (keys, index) = SyncSolValueByMintFreeArgs {
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
        let ix = sync_sol_value_ix_full_for_prog(
            program_id,
            keys,
            index,
            &suffix,
            sol_val_calc_program_id,
        )
        .unwrap();

        let rbh = rpc.get_latest_blockhash().await.unwrap();
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(&payer.pubkey(), &[ix], &[], rbh).unwrap()),
            &[payer.as_ref()],
        )
        .unwrap();

        if !force {
            let should_run = does_tx_modify_pool_state(
                &rpc,
                &tx,
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

        rpc.handle_tx(&tx, args.send_mode).await;
    }
}
