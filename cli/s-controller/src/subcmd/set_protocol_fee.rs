use clap::{ArgGroup, Args};
use s_cli_utils::handle_tx_full;
use s_controller_interface::{set_protocol_fee_ix_with_program_id, SetProtocolFeeIxArgs};
use s_controller_lib::{try_pool_state, SetProtocolFeeFreeArgs};
use sanctum_solana_cli_utils::parse_signer;

use crate::{common::verify_admin, rpc::fetch_pool_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Sets the S controller program's protocol fee rate.

Note: Even though the controller program executes no-op, if none of the fee values was given, the cli program will refuse to execute the instruction.")]
#[clap(group(
    ArgGroup::new("fee")
        .required(true)
        .multiple(true)
))]
pub struct SetProtocolFeeArgs {
    #[arg(
        long,
        short,
        help = "The program's admin authority signer. Defaults to config wallet if not set."
    )]
    pub admin: Option<String>,

    #[arg(
        long,
        short,
        help = "The pool's new trading protocl fee in bips. No change if not set.",
        group = "fee"
    )]
    pub trading_fee: Option<u16>,

    #[arg(
        long,
        short,
        help = "The pool's new lp protocl fee in bips. No change if not set.",
        group = "fee"
    )]
    pub lp_fee: Option<u16>,
}

impl SetProtocolFeeArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            admin,
            trading_fee: new_trading_protocol_fee_bps,
            lp_fee: new_lp_protocol_fee_bps,
        } = match args.subcmd {
            Subcmd::SetProtocolFee(a) => a,
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

        let ix = set_protocol_fee_ix_with_program_id(
            program_id,
            SetProtocolFeeFreeArgs {
                pool_state: pool_state_acc,
            }
            .resolve_for_prog(program_id)
            .unwrap(),
            SetProtocolFeeIxArgs {
                new_trading_protocol_fee_bps,
                new_lp_protocol_fee_bps,
            },
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
