use std::collections::HashSet;

use clap::Args;
use s_cli_utils::handle_tx_full;
use sanctum_solana_cli_utils::PubkeySrc;
use solana_sdk::{
    address_lookup_table::{
        instruction::{create_lookup_table, extend_lookup_table},
        state::AddressLookupTable,
    },
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
};

use crate::subcmd::Subcmd;

/// TODO: can be more depending on if there are other instructions in the transaction
const MAX_ACCS_PER_EXTEND_LUT: usize = 27;

#[derive(Args, Debug)]
#[command(long_about = "Creates a LUT for all the flat fee program's accounts")]
pub struct CreateLutArgs {
    #[arg(
        long,
        short,
        help = "Address of the LUT to create. Extends LUT if already exists. Defaults to newly created derived LUT PDA if not provided."
    )]
    pub lut: Option<String>,
}

impl CreateLutArgs {
    pub async fn run(args: crate::Args) {
        let Self { lut } = match args.subcmd {
            Subcmd::CreateLut(a) => a,
            _ => unreachable!(),
        };

        let signer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let recent_slot = rpc.get_epoch_info().await.unwrap().absolute_slot;
        let (mut this_batch_ixs, lut_addr) = lut.map_or_else(
            || {
                let (ix, pk) = create_lookup_table(signer.pubkey(), signer.pubkey(), recent_slot);
                eprintln!("Will create new LUT {pk}");
                (vec![ix], pk)
            },
            |s| (vec![], PubkeySrc::parse(s.as_str()).unwrap().pubkey()),
        );

        let already_existing_addrs = rpc
            .get_account_with_commitment(&lut_addr, CommitmentConfig::processed())
            .await
            .unwrap()
            .value
            .map_or_else(HashSet::new, |acc| {
                HashSet::from_iter(
                    AddressLookupTable::deserialize(&acc.data)
                        .unwrap()
                        .addresses
                        .iter()
                        .cloned(),
                )
            });

        let to_extend: Vec<Pubkey> = rpc
            .get_program_accounts(&program_id)
            .await
            .unwrap()
            .into_iter()
            .filter_map(|(pk, _acc)| {
                if already_existing_addrs.contains(&pk) {
                    None
                } else {
                    Some(pk)
                }
            })
            .collect();

        for batch in to_extend.chunks(MAX_ACCS_PER_EXTEND_LUT) {
            this_batch_ixs.push(extend_lookup_table(
                lut_addr,
                signer.pubkey(),
                Some(signer.pubkey()),
                Vec::from(batch),
            ));
            let take_this_batch_ixs = std::mem::take(&mut this_batch_ixs);
            handle_tx_full(
                &rpc,
                args.fee_limit_cb,
                args.send_mode,
                take_this_batch_ixs,
                &[],
                &mut [signer.as_ref()],
            )
            .await;
        }
    }
}
