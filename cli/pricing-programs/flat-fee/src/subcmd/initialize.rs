use clap::Args;
use flat_fee_interface::initialize_ix_with_program_id;
use flat_fee_lib::{
    account_resolvers::InitializeFreeArgs, pda::ProgramStateFindPdaArgs, program::STATE_ID,
    utils::try_program_state,
};
use sanctum_solana_cli_utils::TxSendingNonblockingRpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

#[derive(Args, Debug)]
#[command(long_about = "Initializes the flat-fee pricing program's state")]
pub struct InitializeArgs;

impl InitializeArgs {
    pub async fn run(args: crate::Args) {
        let signer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program = args.program;

        let state_pda = ProgramStateFindPdaArgs {
            program_id: program,
        }
        .get_program_state_address_and_bump_seed()
        .0;

        let state = rpc
            .get_account_with_commitment(&state_pda, CommitmentConfig::default())
            .await
            .unwrap();
        if let Some(state) = state.value {
            eprintln!("State PDA {STATE_ID} already initialized:");
            let state = try_program_state(&state.data).unwrap();
            eprintln!("{state:#?}");
            return;
        }

        let ix = initialize_ix_with_program_id(
            program,
            InitializeFreeArgs {
                payer: signer.pubkey(),
            }
            .resolve_for_prog(program),
        )
        .unwrap();

        let rbh = rpc.get_latest_blockhash().await.unwrap();
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(&signer.pubkey(), &[ix], &[], rbh).unwrap()),
            &[signer.as_ref()],
        )
        .unwrap();

        rpc.handle_tx(&tx, args.send_mode).await;
    }
}
