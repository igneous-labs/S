use clap::{ArgGroup, Args};
use s_controller_interface::{set_protocol_fee_ix_with_program_id, SetProtocolFeeIxArgs};
use s_controller_lib::{try_pool_state, SetProtocolFeeFreeArgs};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use crate::{common::verify_admin, rpc::fetch_pool_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Sets the S controller program's protocol fee rate.")]
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
            SetProtocolFeeFreeArgs { pool_state_acc }
                .resolve_for_prog(program_id)
                .unwrap(),
            SetProtocolFeeIxArgs {
                new_trading_protocol_fee_bps,
                new_lp_protocol_fee_bps,
            },
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), admin.as_ref()];
        signers.dedup();

        let rbh = rpc.get_latest_blockhash().await.unwrap();
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(&payer.pubkey(), &[ix], &[], rbh).unwrap()),
            &signers,
        )
        .unwrap();

        rpc.handle_tx(&tx, args.send_mode).await;
    }
}
