use std::collections::HashMap;

use clap::Args;
use flat_fee_lib::{
    pda::{FeeAccountFindPdaArgs, ProgramStateFindPdaArgs},
    utils::{try_fee_account, try_program_state},
};
use sanctum_lst_list::SanctumLst;
use solana_sdk::pubkey::Pubkey;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Views flat-fee pricing program's program state and all fee accounts")]
pub struct ViewArgs;

impl ViewArgs {
    pub async fn run(args: crate::Args) {
        let slsts = args.load_slst_list();
        let Self = match args.subcmd {
            Subcmd::View(a) => a,
            _ => unreachable!(),
        };

        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let state_pda = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;

        let pda_to_lst: HashMap<Pubkey, &SanctumLst> = slsts
            .iter()
            .map(|lst| {
                (
                    FeeAccountFindPdaArgs {
                        program_id,
                        lst_mint: lst.mint,
                    }
                    .get_fee_account_address_and_bump_seed()
                    .0,
                    lst,
                )
            })
            .collect();

        let mut program_accs = rpc.get_program_accounts(&program_id).await.unwrap();
        program_accs.retain(|(pk, acc)| {
            if *pk == state_pda {
                let state = try_program_state(&acc.data).unwrap();
                println!("{state:#?}");
                println!();
                false
            } else {
                true
            }
        });

        for (pk, acc) in program_accs.iter() {
            let symbol = pda_to_lst
                .get(pk)
                .map_or_else(|| "Unknown LST", |SanctumLst { symbol, .. }| symbol);
            println!("{symbol} (PDA {pk}):");
            let fee = try_fee_account(&acc.data).unwrap();
            println!("{fee:#?}");
            println!();
        }

        println!("{} LSTs total", program_accs.len());
    }
}
