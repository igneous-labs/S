use clap::Args;
use flat_fee_interface::{set_lst_fee_ix_with_program_id, SetLstFeeIxArgs, SetLstFeeKeys};
use flat_fee_lib::{
    pda::ProgramStateFindPdaArgs,
    utils::{try_fee_account, try_program_state},
};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_sdk::{
    address_lookup_table::{state::AddressLookupTable, AddressLookupTableAccount},
    hash::Hash,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    signature::Signature,
    transaction::VersionedTransaction,
};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    long_about = "Set the input fee and output fee for all LSTs under the program in one shot.
Assumes manager is a squads multisig and outputs a base58-encoded VersionedTransaction that can be imported into the squads UI."
)]
pub struct BatchSetFeesArgs {
    #[arg(long, short, help = "Pubkey of address lookup table to use")]
    pub lut: Option<String>,

    #[arg(long, short)]
    pub input_fee_bps: i16,

    #[arg(long, short)]
    pub output_fee_bps: i16,
}

impl BatchSetFeesArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            lut,
            input_fee_bps,
            output_fee_bps,
        } = match args.subcmd {
            Subcmd::BatchSetFees(a) => a,
            _ => unreachable!(),
        };

        let signer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let state_pda = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;

        let lut = match lut {
            None => None,
            Some(lut) => {
                let lut = PubkeySrc::parse(&lut).unwrap().pubkey();
                let lut_acc_data = rpc.get_account_data(&lut).await.unwrap();
                Some(AddressLookupTableAccount {
                    key: lut,
                    addresses: AddressLookupTable::deserialize(&lut_acc_data)
                        .unwrap()
                        .addresses
                        .to_vec(),
                })
            }
        };

        let mut program_accs = rpc.get_program_accounts(&program_id).await.unwrap();

        let state_i = program_accs
            .iter()
            .position(|(pk, _acc)| *pk == state_pda)
            .unwrap();
        let (_state_pk, state_acc) = program_accs.remove(state_i);
        let manager = try_program_state(&state_acc.data).unwrap().manager;

        program_accs.retain(|(_pk, acc)| {
            let fee_acc = try_fee_account(&acc.data).unwrap();
            fee_acc.input_fee_bps != input_fee_bps || fee_acc.output_fee_bps != output_fee_bps
        });

        let ixs: Vec<Instruction> = program_accs
            .iter()
            .map(|(pk, _acc)| {
                set_lst_fee_ix_with_program_id(
                    program_id,
                    SetLstFeeKeys {
                        manager,
                        fee_acc: *pk,
                        state: state_pda,
                    },
                    SetLstFeeIxArgs {
                        input_fee_bps,
                        output_fee_bps,
                    },
                )
                .unwrap()
            })
            .collect();
        let message = VersionedMessage::V0(
            Message::try_compile(
                &signer.pubkey(),
                &ixs,
                lut.as_ref().map(std::slice::from_ref).unwrap_or_default(),
                Hash::default(),
            )
            .unwrap(),
        );
        let tx = VersionedTransaction {
            // dummy signature. 1 for signer, 1 for manager
            signatures: vec![Signature::default(); 2],
            message,
        };

        eprintln!(
            "{}",
            bs58::encode(bincode::serialize(&tx).unwrap()).into_string()
        );
    }
}
