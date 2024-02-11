use clap::Args;
use s_controller_interface::LstState;
use s_controller_lib::{
    create_pool_reserves_address_with_pool_state_id,
    create_protocol_fee_accumulator_address_with_protocol_fee_id, find_lst_state_list_address,
    find_pool_state_address, find_protocol_fee_address, try_lst_state_list, try_pool_state, U8Bool,
};
use sanctum_token_lib::{mint_supply, token_account_balance};
use solana_sdk::native_token::lamports_to_sol;

use crate::common::find_sanctum_lst_by_mint;

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "View info about the pool.")]
pub struct ViewArgs;

impl ViewArgs {
    pub async fn run(args: crate::Args) {
        let Self = match args.subcmd {
            Subcmd::View(a) => a,
            _ => unreachable!(),
        };

        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let pool_state_addr = find_pool_state_address(program_id).0;
        let lst_state_list_addr = find_lst_state_list_address(program_id).0;

        let mut main_accs = rpc
            .get_multiple_accounts(&[pool_state_addr, lst_state_list_addr])
            .await
            .unwrap();
        let lst_state_list_acc = main_accs.pop().unwrap().unwrap();
        let pool_state_acc = main_accs.pop().unwrap().unwrap();
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        let lst_state_list = try_lst_state_list(&lst_state_list_acc.data).unwrap();
        eprintln!("{pool_state:#?}");
        eprintln!();
        eprintln!("{lst_state_list:#?}");
        eprintln!();
        let protocol_fee_id = find_protocol_fee_address(program_id).0;
        for lst_state in lst_state_list {
            let LstState {
                mint,
                is_input_disabled,
                sol_value,
                sol_value_calculator,
                ..
            } = lst_state;
            let sanctum_lst_opt = find_sanctum_lst_by_mint(*mint);
            eprintln!(
                "{}:",
                sanctum_lst_opt.map_or_else(|| mint.to_string(), |lst| lst.symbol.clone())
            );
            eprintln!(
                "is_input_disabled: {}",
                U8Bool(*is_input_disabled).is_true()
            );
            eprintln!("sol_value: {}", lamports_to_sol(*sol_value));
            eprintln!("sol_value_calculator: {sol_value_calculator}");
            let token_program = match sanctum_lst_opt {
                Some(s) => std::future::ready(s.token_program).await,
                None => async { rpc.get_account(mint).await.unwrap().owner }.await,
            };
            let reserves_addr = create_pool_reserves_address_with_pool_state_id(
                pool_state_addr,
                lst_state,
                token_program,
            )
            .unwrap();
            let protocol_fee_accum_addr =
                create_protocol_fee_accumulator_address_with_protocol_fee_id(
                    protocol_fee_id,
                    lst_state,
                    token_program,
                )
                .unwrap();
            let mut token_accs = rpc
                .get_multiple_accounts(&[reserves_addr, protocol_fee_accum_addr])
                .await
                .unwrap();
            let protocol_fee_accum_acc = token_accs.pop().unwrap().unwrap();
            let reserves_acc = token_accs.pop().unwrap().unwrap();
            eprintln!(
                "reserves {reserves_addr}: {}",
                lamports_to_sol(token_account_balance(reserves_acc).unwrap())
            );
            eprintln!(
                "protocol fees {protocol_fee_accum_addr}: {}",
                lamports_to_sol(token_account_balance(protocol_fee_accum_acc).unwrap())
            );
            eprintln!();
        }
        let lp_mint_acc = rpc.get_account(&pool_state.lp_token_mint).await.unwrap();
        eprintln!(
            "LP token supply: {}",
            lamports_to_sol(mint_supply(lp_mint_acc).unwrap())
        );
    }
}
