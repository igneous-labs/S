use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use flat_fee_interface::{add_lst_ix_with_program_id, AddLstIxArgs};
use flat_fee_lib::{
    account_resolvers::AddLstFreeArgs, pda::ProgramStateFindPdaArgs, utils::try_program_state,
};
use s_cli_utils::handle_tx_full;
use sanctum_solana_cli_utils::parse_signer;
use solana_readonly_account::sdk::KeyedAccount;

use crate::lst_arg::LstArg;

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

    #[arg(
        help = "Mint of the new LST to add. Can either be a pubkey or case-insensitive symbol of a token on sanctum-lst-list. e.g. 'bsol'",
        value_parser = StringValueParser::new().try_map(|s| LstArg::parse_arg(&s)),
    )]
    pub lst_mint: LstArg,

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

        let ix = add_lst_ix_with_program_id(
            program_id,
            AddLstFreeArgs {
                payer: payer.pubkey(),
                state_acc: KeyedAccount {
                    pubkey: state_pda,
                    account: state_acc,
                },
                lst_mint: lst_mint.mint(),
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
