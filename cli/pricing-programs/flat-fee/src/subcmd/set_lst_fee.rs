use clap::Args;
use flat_fee_interface::{set_lst_fee_ix_with_program_id, SetLstFeeIxArgs};
use flat_fee_lib::{
    account_resolvers::SetLstFeeByMintFreeArgs, pda::ProgramStateFindPdaArgs,
    utils::try_program_state,
};
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_readonly_account::sdk::KeyedAccount;

use crate::lst_arg::LstArg;

use super::{common::verify_manager, Subcmd};

#[derive(Args, Debug)]
#[command(long_about = "Update the fees for given LST")]
pub struct SetLstFeeArgs {
    #[arg(
        long,
        short,
        help = "The program's current manager signer. Defaults to config wallet if not set."
    )]
    pub manager: Option<String>,

    #[arg(
        help = "Mint of the LST to set fees for. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'"
    )]
    pub lst_mint: String,

    #[arg(help = "Fee in bips to impose when the LST is used as input")]
    pub input_fee_bps: i16,

    #[arg(help = "Fee in bips to impose when the LST is used as output")]
    pub output_fee_bps: i16,
}

impl SetLstFeeArgs {
    pub async fn run(args: crate::Args) {
        let slsts = args.load_slst_list();
        let Self {
            manager,
            lst_mint,
            input_fee_bps,
            output_fee_bps,
        } = match args.subcmd {
            Subcmd::SetLstFee(a) => a,
            _ => unreachable!(),
        };
        let lst_mint = LstArg::parse_arg(&lst_mint, &slsts).unwrap();
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

        let ix = set_lst_fee_ix_with_program_id(
            program_id,
            SetLstFeeByMintFreeArgs {
                lst_mint: lst_mint.mint(),
                state_acc: KeyedAccount {
                    pubkey: state_pda,
                    account: state_acc,
                },
            }
            .resolve_for_prog(program_id)
            .unwrap(),
            SetLstFeeIxArgs {
                input_fee_bps,
                output_fee_bps,
            },
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
