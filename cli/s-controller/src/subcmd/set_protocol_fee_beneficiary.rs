use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use s_controller_interface::set_protocol_fee_beneficiary_ix_with_program_id;
use s_controller_lib::{
    find_pool_state_address, try_pool_state, SetProtocolFeeBeneficiaryFreeArgs,
};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    transaction::VersionedTransaction,
};

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

        let curr_beneficiary_signer = curr_beneficiary.map(|s| parse_signer(&s).unwrap());
        let curr_beneficiary = curr_beneficiary_signer.as_ref().unwrap_or(&payer);

        let pool_state_acc = fetch_pool_state(&rpc, program_id).await;
        let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
        verify_protocol_fee_beneficiary(pool_state, curr_beneficiary.pubkey()).unwrap();

        let ix = set_protocol_fee_beneficiary_ix_with_program_id(
            program_id,
            SetProtocolFeeBeneficiaryFreeArgs {
                new_beneficiary,
                pool_state: KeyedAccount {
                    pubkey: find_pool_state_address(program_id).0,
                    account: pool_state_acc,
                },
            }
            .resolve_for_prog(program_id)
            .unwrap(),
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), curr_beneficiary.as_ref()];
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
