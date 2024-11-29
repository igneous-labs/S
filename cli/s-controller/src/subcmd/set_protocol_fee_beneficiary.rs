use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use s_controller_interface::set_protocol_fee_beneficiary_ix_with_program_id;
use s_controller_lib::{try_pool_state, SetProtocolFeeBeneficiaryFreeArgs};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_sdk::pubkey::Pubkey;

use crate::{common::verify_protocol_fee_beneficiary, rpc::fetch_pool_state};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Sets the pool's protocol fee beneficiary.")]
pub struct SetProtocolFeeBeneficiaryArgs {
    #[arg(
        long,
        short,
        help = "The pool's protocol fee beneficiary signer. Defaults to config wallet if not set."
    )]
    pub curr_beneficiary: Option<String>,

    #[arg(
        help = "The pool's new protocol fee beneficiary to set.",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s))
    )]
    pub new_beneficiary: Pubkey,
}

impl SetProtocolFeeBeneficiaryArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            curr_beneficiary,
            new_beneficiary,
        } = match args.subcmd {
            Subcmd::SetProtocolFeeBeneficiary(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let curr_beneficiary_signer =
            curr_beneficiary.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let curr_beneficiary = curr_beneficiary_signer.as_ref().unwrap_or(&payer);

        let pool_state_acc = fetch_pool_state(&rpc, program_id).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        verify_protocol_fee_beneficiary(pool_state, curr_beneficiary.pubkey()).unwrap();

        let ix = set_protocol_fee_beneficiary_ix_with_program_id(
            program_id,
            SetProtocolFeeBeneficiaryFreeArgs {
                new_beneficiary,
                pool_state: pool_state_acc,
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
            &mut [payer.as_ref(), curr_beneficiary.as_ref()],
        )
        .await;
    }
}
