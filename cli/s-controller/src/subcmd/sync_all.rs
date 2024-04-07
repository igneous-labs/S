use clap::Args;
use s_cli_utils::handle_tx_full;
use s_controller_interface::LstState;
use s_controller_lib::{
    find_lst_state_list_address, find_pool_state_address, sync_sol_value_ix_full_for_prog,
    try_lst_state_list, SyncSolValueByMintFreeArgs, SyncSolValuePdas,
};
use sanctum_lst_list::SanctumLst;
use sanctum_solana_client_utils::to_est_cu_sim_tx;
use sanctum_token_lib::MintWithTokenProgram;
use solana_readonly_account::keyed::Keyed;
use solana_sdk::instruction::Instruction;

use crate::{
    common::{find_sanctum_lst_by_mint, sol_value_calculator_accounts_of_sanctum_lst},
    rpc::does_tx_modify_pool_state,
};

use super::Subcmd;

// TODO: this can probably increase if we use a LUT
const MAX_GENERIC_SOL_VAL_CALC_SYNCS_PER_TX: usize = 4;

#[derive(Args, Debug)]
#[command(
    about = "Sync SOL value for all sanctum-lst-list LSTs in the pool",
    long_about = "Sync SOL value for all sanctum-lst-list LSTs in the pool.
To sync a single LST that might not be on sanctum-lst-list, use sync."
)]
pub struct SyncAllArgs {
    #[arg(
        long,
        short,
        help = "If flag set, will process tx even if pool does not need to sync. Otherwise, simulates sync transactions first and does nothing if no changes.",
        default_value_t = false
    )]
    pub force: bool,
}

impl SyncAllArgs {
    pub async fn run(args: crate::Args) {
        let Self { force } = match args.subcmd {
            Subcmd::SyncAll(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let lst_state_list_addr = find_lst_state_list_address(program_id).0;
        let pool_state_addr = find_pool_state_address(program_id).0;

        let mut fetched_accs = rpc
            .get_multiple_accounts(&[lst_state_list_addr, pool_state_addr])
            .await
            .unwrap();
        let pool_state_acc = fetched_accs.pop().unwrap().unwrap();
        let lst_state_list_acc = fetched_accs.pop().unwrap().unwrap();

        let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();

        let sanctum_lsts: Vec<&SanctumLst> = lst_state_list
            .iter()
            .filter_map(|LstState { mint, .. }| {
                let res = find_sanctum_lst_by_mint(*mint);
                if res.is_none() {
                    eprintln!("{mint} not on sanctum-lst-list, skipping");
                }
                res
            })
            .collect();

        // dyn Signer is not Sync, so just send the txes sequentially
        let fut_iter = sanctum_lsts
            .chunks(MAX_GENERIC_SOL_VAL_CALC_SYNCS_PER_TX)
            .map(|chunk| async {
                let ixs: Vec<Instruction> = chunk
                    .iter()
                    .map(|sanctum_lst| {
                        let (keys, index, sol_value_calculator_program_id) =
                            SyncSolValueByMintFreeArgs {
                                lst_state_list: &lst_state_list_acc,
                                lst_mint: MintWithTokenProgram {
                                    pubkey: sanctum_lst.mint,
                                    token_program: sanctum_lst.token_program,
                                },
                            }
                            .resolve_with_pdas(SyncSolValuePdas {
                                pool_state: pool_state_addr,
                                lst_state_list: lst_state_list_addr,
                            })
                            .unwrap();
                        sync_sol_value_ix_full_for_prog(
                            program_id,
                            keys,
                            index,
                            &sol_value_calculator_accounts_of_sanctum_lst(sanctum_lst),
                            sol_value_calculator_program_id,
                        )
                        .unwrap()
                    })
                    .collect();
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
                        eprint!("Sync not required for ");
                        for sanctum_lst in chunk.iter() {
                            eprint!("{}, ", sanctum_lst.symbol);
                        }
                        eprintln!();
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
            });
        for fut in fut_iter {
            fut.await;
        }
    }
}

#[cfg(test)]
mod tests {
    use generic_pool_calculator_interface::SOL_TO_LST_IX_ACCOUNTS_LEN;
    use s_controller_interface::SyncSolValueKeys;
    use sanctum_solana_test_utils::assert_tx_with_cb_ixs_within_size_limits;
    use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

    use super::*;

    #[test]
    fn max_generic_sol_val_calc_syncs_per_tx_does_not_exceed_tx_size() {
        let program_id = Pubkey::new_unique();
        let base_keys = SyncSolValueKeys {
            lst_mint: Pubkey::default(),
            pool_state: Pubkey::new_unique(),
            lst_state_list: Pubkey::new_unique(),
            pool_reserves: Pubkey::new_unique(),
        };
        let ixs: Vec<Instruction> = (0..MAX_GENERIC_SOL_VAL_CALC_SYNCS_PER_TX)
            .map(|_| {
                let lst_mint = Pubkey::new_unique();
                let lst_index = 0;
                let mut accounts = base_keys;
                accounts.lst_mint = lst_mint;
                let sol_value_calculator_accounts: [AccountMeta; SOL_TO_LST_IX_ACCOUNTS_LEN] =
                    [0; SOL_TO_LST_IX_ACCOUNTS_LEN].map(|_| AccountMeta {
                        pubkey: Pubkey::new_unique(),
                        is_signer: false,
                        is_writable: false,
                    });
                sync_sol_value_ix_full_for_prog(
                    program_id,
                    accounts,
                    lst_index,
                    &sol_value_calculator_accounts,
                    Pubkey::new_unique(),
                )
                .unwrap()
            })
            .collect();
        assert_tx_with_cb_ixs_within_size_limits(&Pubkey::new_unique(), ixs.into_iter(), &[]);
    }
}
