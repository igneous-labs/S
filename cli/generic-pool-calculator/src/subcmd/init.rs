use clap::Args;
use generic_pool_calculator_interface::init_ix_with_program_id;
use generic_pool_calculator_lib::{
    account_resolvers::InitFreeArgs, pda::CalculatorStateFindPdaArgs, utils::try_calculator_state,
};
use s_cli_utils::handle_tx_full;
use solana_sdk::commitment_config::CommitmentConfig;

#[derive(Args, Debug)]
#[command(long_about = "Initializes the SOL value calculator program's state")]
pub struct InitArgs;

impl InitArgs {
    pub async fn run(args: crate::Args) {
        let signer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program.program_id();

        let state_pda = CalculatorStateFindPdaArgs { program_id }
            .get_calculator_state_address_and_bump_seed()
            .0;

        let state = rpc
            .get_account_with_commitment(&state_pda, CommitmentConfig::confirmed())
            .await
            .unwrap();
        if let Some(state) = state.value {
            eprintln!("State PDA {state_pda} already initialized:");
            let state = try_calculator_state(&state.data).unwrap();
            eprintln!("{state:#?}");
            return;
        }

        let ix = init_ix_with_program_id(
            program_id,
            InitFreeArgs {
                payer: signer.pubkey(),
            }
            .resolve_for_prog(program_id),
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
