use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use flat_fee_interface::remove_lst_ix_with_program_id;
use flat_fee_lib::{
    account_resolvers::RemoveLstFreeArgs, pda::ProgramStateFindPdaArgs, utils::try_program_state,
};
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_readonly_account::sdk::KeyedAccount;

use crate::lst_arg::LstArg;

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

    #[arg(
        help = "Mint of the LST to remove. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub lst_mint: LstArg,

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

        let manager_signer =
            manager.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let manager = manager_signer.as_ref().unwrap_or(&payer);

        let state_pda = ProgramStateFindPdaArgs { program_id }
            .get_program_state_address_and_bump_seed()
            .0;
        let state_acc = rpc.get_account(&state_pda).await.unwrap();
        let state = try_program_state(&state_acc.data).unwrap();
        verify_manager(state, manager.pubkey()).unwrap();

        let refund_rent_to = PubkeySrc::parse(&refund_rent_to).unwrap();

        let ix = remove_lst_ix_with_program_id(
            program_id,
            RemoveLstFreeArgs {
                refund_rent_to: refund_rent_to.pubkey(),
                lst_mint: lst_mint.mint(),
                state_acc: KeyedAccount {
                    pubkey: state_pda,
                    account: state_acc,
                },
            }
            .resolve_for_prog(program_id)
            .unwrap(),
        )
        .unwrap();

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            vec![ix],
            &[],
            &mut [payer.as_ref(), manager.as_ref()],
        )
        .await;
    }
}
