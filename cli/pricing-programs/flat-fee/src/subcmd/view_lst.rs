use clap::Args;
use flat_fee_lib::{pda::FeeAccountFindPdaArgs, utils::try_fee_account};

use crate::{lst_arg::LstArg, subcmd::Subcmd};

#[derive(Args, Debug)]
#[command(long_about = "View the current fees for a given LST")]
pub struct ViewLstArgs {
    #[arg(
        help = "Mint of the LST to view fees for. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'"
    )]
    pub lst_mint: String,
}

impl ViewLstArgs {
    pub async fn run(args: crate::Args) {
        let slsts = args.load_slst_list();
        let Self { lst_mint } = match args.subcmd {
            Subcmd::ViewLst(a) => a,
            _ => unreachable!(),
        };
        let lst_mint = LstArg::parse_arg(&lst_mint, &slsts).unwrap();

        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let fee_account_pda = FeeAccountFindPdaArgs {
            program_id,
            lst_mint: lst_mint.mint(),
        }
        .get_fee_account_address_and_bump_seed()
        .0;
        let fee_account_data = rpc.get_account_data(&fee_account_pda).await.unwrap();
        let fee_account = try_fee_account(&fee_account_data).unwrap();

        println!("{fee_account:#?}");
    }
}
