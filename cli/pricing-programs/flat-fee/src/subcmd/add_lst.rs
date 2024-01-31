use clap::Args;
use flat_fee_interface::{add_lst_ix_with_program_id, AddLstIxArgs};
use flat_fee_lib::{
    account_resolvers::AddLstFreeArgs, pda::ProgramStateFindPdaArgs, utils::try_program_state,
};
use sanctum_solana_cli_utils::{parse_pubkey_src, parse_signer, TxSendingNonblockingRpcClient};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::{common::verify_manager, Subcmd};

#[derive(Args, Debug)]
#[command(long_about = "Enable an LST to be supported by the flat-fee pricing program")]
pub struct AddLstArgs {
    #[arg(
        long,
        short,
        help = "The program's current manager signer. Defaults to config wallet if not set."
    )]
    pub manager: Option<String>,

    #[arg(help = "The mint of LST")]
    pub lst_mint: String,

    #[arg(help = "Fee in bips to impose when the LST is used as input")]
    pub input_fee_bps: i16,

    #[arg(help = "Fee in bips to impose when the LST is used as output")]
    pub output_fee_bps: i16,
}

impl AddLstArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            manager,
            lst_mint,
            input_fee_bps,
            output_fee_bps,
        } = match args.subcmd {
            Subcmd::AddLst(a) => a,
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

        let lst_mint = parse_pubkey_src(&lst_mint).unwrap().pubkey();

        let ix = add_lst_ix_with_program_id(
            program_id,
            AddLstFreeArgs {
                payer: payer.pubkey(),
                state_acc: KeyedAccount {
                    pubkey: state_pda,
                    account: state_acc,
                },
                lst_mint,
            }
            .resolve_for_prog(program_id)
            .unwrap()
            .0,
            AddLstIxArgs {
                input_fee_bps,
                output_fee_bps,
            },
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
