use clap::Args;
use generic_pool_calculator_lib::{pda::CalculatorStateFindPdaArgs, utils::try_calculator_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Views the generic pool calculator's state (manager + last_upgrade_slot)")]
pub struct ViewArgs;

impl ViewArgs {
    pub async fn run(args: crate::Args) {
        let Self = match args.subcmd {
            Subcmd::View(a) => a,
            _ => unreachable!(),
        };
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program.program_id();
        let state_pda = CalculatorStateFindPdaArgs { program_id }
            .get_calculator_state_address_and_bump_seed()
            .0;
        let state_data = rpc.get_account_data(&state_pda).await.unwrap();
        let state = try_calculator_state(&state_data).unwrap();

        println!("{state:#?}");
    }
}
