use clap::Args;
use flat_fee_lib::{pda::ProgramStateFindPdaArgs, utils::try_program_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Views flat-fee pricing program's program state")]
pub struct ViewArgs;

impl ViewArgs {
    pub async fn run(args: crate::Args) {
        let Self = match args.subcmd {
            Subcmd::View(a) => a,
            _ => unreachable!(),
        };

        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let state_pda = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;
        let state_data = rpc.get_account_data(&state_pda).await.unwrap();
        let state = try_program_state(&state_data).unwrap();

        println!("{state:#?}");
    }
}
