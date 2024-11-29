use clap::Args;
use flat_fee_interface::{set_lp_withdrawal_fee_ix_with_program_id, SetLpWithdrawalFeeIxArgs};
use flat_fee_lib::{
    account_resolvers::SetLpWithdrawalFeeFreeArgs, pda::ProgramStateFindPdaArgs,
    utils::try_program_state,
};
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_readonly_account::sdk::KeyedAccount;

use super::{common::verify_manager, Subcmd};

#[derive(Args, Debug)]
#[command(long_about = "Update the fees imposed for redeeming LP token for LST")]
pub struct SetLpWithdrawalFeeArgs {
    #[arg(
        long,
        short,
        help = "The program's current manager signer. Defaults to config wallet if not set."
    )]
    pub manager: Option<String>,

    #[arg(help = "Fee in bips to impose when redeeming LP token for LST")]
    pub lp_withdrawal_fee_bps: u16,
}

impl SetLpWithdrawalFeeArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            manager,
            lp_withdrawal_fee_bps,
        } = match args.subcmd {
            Subcmd::SetLpWithdrawalFee(a) => a,
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

        let ix = set_lp_withdrawal_fee_ix_with_program_id(
            program_id,
            SetLpWithdrawalFeeFreeArgs {
                state_acc: KeyedAccount {
                    pubkey: state_pda,
                    account: state_acc,
                },
            }
            .resolve_for_prog(program_id)
            .unwrap(),
            SetLpWithdrawalFeeIxArgs {
                lp_withdrawal_fee_bps,
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
