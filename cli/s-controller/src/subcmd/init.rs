use clap::Args;
use s_controller_interface::initialize_ix_with_program_id;
use s_controller_lib::{InitializeFreeArgs, InitializeResolveForProg};
use sanctum_solana_cli_utils::{parse_pubkey_src, parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Initializes the S controller program's state.

Prerequisites:
- lp_token_mint must be an initialized spl-token (not 2022) mint with 9 decimals and 0 supply and have mint authority set to the program's initial authority

The procedure will:
- Initialize the pool state only. Use add-lst to initialize the lst list and add the first LST to the pool.
- Transfer the mint authority of lp_token_mint to the program. Make sure token metadata is already set up if required.
- Set pool manager and rebalance authority to the program's initial authority
")]
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

        let lp_token_mint = parse_pubkey_src(&lp_token_mint).unwrap();

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

        let mut signers = vec![payer.as_ref(), init_auth.as_ref()];
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
