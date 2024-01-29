use clap::Args;
use flat_fee_interface::remove_lst_ix_with_program_id;
use flat_fee_lib::{
    account_resolvers::RemoveLstFreeArgs, pda::ProgramStateFindPdaArgs, utils::try_program_state,
};
use sanctum_solana_cli_utils::{parse_pubkey_src, parse_signer, TxSendingNonblockingRpcClient};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::{common::verify_manager, Subcmd};

#[derive(Args, Debug)]
#[command(long_about = "Disable an added LST")]
pub struct RemoveLstArgs {
    #[arg(
        long,
        short,
        help = "The program's current manager signer. Defaults to config wallet if not set."
    )]
    pub manager: Option<String>,

    #[arg(help = "The mint of LST")]
    pub lst_mint: String,

    #[arg(help = "Account to refund SOL rent to")]
    pub refund_rent_to: String,
}

impl RemoveLstArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            manager,
            lst_mint,
            refund_rent_to,
        } = match args.subcmd {
            Subcmd::RemoveLst(a) => a,
            _ => unreachable!(),
        };
        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let manager_signer = manager.map(|s| parse_signer(&s).unwrap());
        let manager = manager_signer.as_ref().unwrap_or(&payer);

        let state_pda = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;
        let state_acc = rpc.get_account(&state_pda).await.unwrap();
        let state = try_program_state(&state_acc.data).unwrap();
        verify_manager(state, manager.pubkey()).unwrap();

        let lst_mint = parse_pubkey_src(&lst_mint).unwrap();
        let refund_rent_to = parse_pubkey_src(&refund_rent_to).unwrap();

        let ix = remove_lst_ix_with_program_id(
            program_id,
            RemoveLstFreeArgs {
                refund_rent_to: refund_rent_to.pubkey(),
                lst_mint: lst_mint.pubkey(),
                state_acc: KeyedAccount {
                    pubkey: state_pda,
                    account: state_acc,
                },
            }
            .resolve() // TODO: resolve_with_prog(program_id)
            .unwrap(),
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), manager.as_ref()];
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
