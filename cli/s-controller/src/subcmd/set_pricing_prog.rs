use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_cli_utils::handle_tx_full;
use s_controller_interface::set_pricing_program_ix_with_program_id;
use s_controller_lib::{try_pool_state, SetPricingProgramFreeArgs};
use sanctum_solana_cli_utils::parse_signer;

use crate::{common::verify_admin, pricing_prog_arg::PricingProgArg, rpc::fetch_pool_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Sets the S controller program's pricing program.")]
pub struct SetPricingProgArgs {
    #[arg(
        long,
        short,
        help = "The program's admin authority signer. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,

    #[arg(
        help = "The new pricing program to set to. This can be a pubkey or the following known pricing program identifiers:
- flat-fee",
    value_parser = StringValueParser::new().try_map(|s| PricingProgArg::parse_arg(&s)))]
    pub pricing_prog: PricingProgArg,
}

impl SetPricingProgArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            admin,
            pricing_prog,
        } = match args.subcmd {
            Subcmd::SetPricingProg(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let admin_signer = admin.map(|s| parse_signer(&s).unwrap());
        let admin = admin_signer.as_ref().unwrap_or(&payer);

        let pool_state_acc = fetch_pool_state(&rpc, program_id).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        verify_admin(pool_state, admin.pubkey()).unwrap();

        let ix = set_pricing_program_ix_with_program_id(
            program_id,
            SetPricingProgramFreeArgs {
                new_pricing_program: pricing_prog.program_id(),
                pool_state_acc,
            }
            .resolve_for_prog(program_id)
            .unwrap(),
        )
        .unwrap();

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            vec![ix],
            &[],
            &mut [payer.as_ref(), admin.as_ref()],
        )
        .await;
    }
}
