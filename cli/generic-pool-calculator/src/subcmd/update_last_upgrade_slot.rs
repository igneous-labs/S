use std::str::FromStr;

use clap::{
    builder::{StringValueParser, TypedValueParser},
    Args,
};
use generic_pool_calculator_interface::{
    update_last_upgrade_slot_ix_with_program_id, UpdateLastUpgradeSlotKeys,
};
use generic_pool_calculator_lib::{
    pda::CalculatorStateFindPdaArgs,
    utils::{read_stake_pool_progdata_meta, try_calculator_state},
};
use s_cli_utils::{handle_tx_full, pubkey_src_to_box_dyn_signer};
use sanctum_solana_cli_utils::PubkeySrc;
use solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig};
use solana_rpc_client_api::config::RpcAccountInfoConfig;
use solana_sdk::{bpf_loader_upgradeable, pubkey::Pubkey};

use super::{common::verify_manager, Subcmd};

#[derive(Args, Debug)]
#[command(
    long_about = "Updates the SOL value calculator program's pool program last upgrade slot to the current one."
)]
pub struct UpdateLastUpgradeSlotArgs {
    #[arg(
        long,
        short,
        help = "The program's current manager signer. Defaults to config wallet if not set."
    )]
    pub curr_manager: Option<String>,

    #[arg(
        help = "Pubkey of the pool program this calculator program works for.",
        value_parser = StringValueParser::new().try_map(|s| Pubkey::from_str(&s)),
    )]
    pub pool_program_id: Pubkey,
}

impl UpdateLastUpgradeSlotArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            curr_manager,
            pool_program_id,
        } = match args.subcmd {
            Subcmd::UpdateLastUpgradeSlot(a) => a,
            _ => unreachable!(),
        };
        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program.program_id();

        let curr_manager_signer =
            curr_manager.map(|s| pubkey_src_to_box_dyn_signer(PubkeySrc::parse(&s).unwrap()));
        let curr_manager = curr_manager_signer.as_ref().unwrap_or(&payer);

        let state_pda = CalculatorStateFindPdaArgs { program_id }
            .get_calculator_state_address_and_bump_seed()
            .0;
        let state_data = rpc.get_account_data(&state_pda).await.unwrap();
        let state = try_calculator_state(&state_data).unwrap();
        verify_manager(state, curr_manager.pubkey()).unwrap();

        // programdata addr is BpfLoader PDA [program_addr]:
        // https://docs.rs/solana-program/latest/src/solana_program/bpf_loader_upgradeable.rs.html#211
        let (pool_progdata_addr, _) =
            Pubkey::find_program_address(&[pool_program_id.as_ref()], &bpf_loader_upgradeable::ID);
        let pool_progdata = rpc
            .get_account_with_config(
                &pool_progdata_addr,
                RpcAccountInfoConfig {
                    data_slice: Some(UiDataSliceConfig {
                        offset: 0,
                        length: bpf_loader_upgradeable::UpgradeableLoaderState::size_of_programdata_metadata(),
                    }),
                    // must use base64 otherwise `Encoded binary (base 58) data should be less than 128 bytes, please use Base64 encoding.`
                    // idk why base64 isnt default
                    encoding: Some(UiAccountEncoding::Base64),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
            .value
            .unwrap();
        let (last_update_slot, _) = read_stake_pool_progdata_meta(pool_progdata).unwrap();
        if state.last_upgrade_slot == last_update_slot {
            eprint!("Already at latest last upgrade slot {last_update_slot}");
            return;
        }

        let ix = update_last_upgrade_slot_ix_with_program_id(
            program_id,
            UpdateLastUpgradeSlotKeys {
                manager: curr_manager.pubkey(),
                state: state_pda,
                pool_program: pool_program_id,
                pool_program_data: pool_progdata_addr,
            },
        )
        .unwrap();

        handle_tx_full(
            &rpc,
            args.fee_limit_cb,
            args.send_mode,
            vec![ix],
            &[],
            &mut [payer.as_ref(), curr_manager.as_ref()],
        )
        .await;
    }
}
