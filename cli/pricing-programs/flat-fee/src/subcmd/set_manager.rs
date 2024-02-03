use clap::Args;
use flat_fee_interface::{set_manager_ix_with_program_id, SetManagerKeys};
use flat_fee_lib::{pda::ProgramStateFindPdaArgs, utils::try_program_state};
use sanctum_solana_cli_utils::{parse_pubkey_src, parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::{common::verify_manager, Subcmd};

#[derive(Args, Debug)]
#[command(long_about = "Sets the flat-fee pricing program's manager")]
pub struct SetManagerArgs {
    #[arg(
        long,
        short,
        help = "The program's current manager signer. Defaults to config wallet if not set."
    )]
    pub curr_manager: Option<String>,

    #[arg(help = "The new program's manager to set. Can be a pubkey or signer.")]
    pub new_manager: String,
}

impl SetManagerArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            curr_manager,
            new_manager,
        } = match args.subcmd {
            Subcmd::SetManager(a) => a,
            _ => unreachable!(),
        };
        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let curr_manager_signer = curr_manager.map(|s| parse_signer(&s).unwrap());
        let curr_manager = curr_manager_signer.as_ref().unwrap_or(&payer);

        let new_manager = parse_pubkey_src(&new_manager).unwrap();
        let state_pda = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;
        let state_data = rpc.get_account_data(&state_pda).await.unwrap();
        let state = try_program_state(&state_data).unwrap();
        verify_manager(state, curr_manager.pubkey()).unwrap();

        let ix = set_manager_ix_with_program_id(
            program_id,
            SetManagerKeys {
                current_manager: state.manager,
                new_manager: new_manager.pubkey(),
                state: state_pda,
            },
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), curr_manager.as_ref()];
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
