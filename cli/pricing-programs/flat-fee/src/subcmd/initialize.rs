use clap::Args;
use flat_fee_interface::initialize_ix_with_program_id;
use flat_fee_lib::{
    account_resolvers::InitializeFreeArgs, pda::ProgramStateFindPdaArgs, utils::try_program_state,
};
use s_cli_utils::handle_tx_full;
use solana_sdk::commitment_config::CommitmentConfig;

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
            eprintln!("State PDA {state_pda} already initialized:");
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

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            vec![ix],
            &[],
            &mut [signer.as_ref()],
        )
        .await;
    }
}
