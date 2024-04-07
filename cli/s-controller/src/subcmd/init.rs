use clap::Args;
use s_cli_utils::handle_tx_full;
use s_controller_interface::initialize_ix_with_program_id;
use s_controller_lib::{InitializeFreeArgs, InitializeResolveForProg};
use sanctum_solana_cli_utils::{parse_signer, PubkeySrc};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(
    about = "Initializes the S controller program's state",
    long_about = "Initializes the S controller program's state

Prerequisites:
- lp_token_mint must be an initialized spl-token (not 2022) mint with 9 decimals and 0 supply and have mint authority set to the program's initial authority

The procedure will:
- Initialize the pool state only. Use add-lst to initialize the lst list and add the first LST to the pool.
- Transfer the mint authority of lp_token_mint to the program. Make sure token metadata is already set up if required.
- Set pool manager and rebalance authority to the program's initial authority"
)]
pub struct InitArgs {
    #[arg(
        long,
        short,
        help = "The program's initial authority. Defaults to config wallet if not set."
    )]
    pub init_auth: Option<String>,

    #[arg(help = "The initialized lp_token_mint. Can be a pubkey or signer.")]
    pub lp_token_mint: String,
}

impl InitArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            init_auth,
            lp_token_mint,
        } = match args.subcmd {
            Subcmd::Init(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let init_auth_signer = init_auth.map(|s| parse_signer(&s).unwrap());
        let init_auth = init_auth_signer.as_ref().unwrap_or(&payer);

        let lp_token_mint = PubkeySrc::parse(&lp_token_mint).unwrap();

        let ix = initialize_ix_with_program_id(
            program_id,
            InitializeFreeArgs {
                payer: payer.pubkey(),
                lp_token_mint: lp_token_mint.pubkey(),
            }
            .resolve_for_prog(InitializeResolveForProg {
                program_id,
                initial_authority: init_auth.pubkey(),
            }),
        )
        .unwrap();

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            vec![ix],
            &[],
            &mut [payer.as_ref(), init_auth.as_ref()],
        )
        .await;
    }
}
