use clap::Args;
use s_controller_interface::{disable_pool_ix_with_program_id, DisablePoolKeys};
use s_controller_lib::{
    find_disable_pool_authority_list_address, find_pool_state_address,
    try_disable_pool_authority_list, try_pool_state,
};
use sanctum_solana_cli_utils::{parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::{
    common::{verify_disable_pool_authority, verify_pool_is_not_rebalancing},
    Subcmd,
};

#[derive(Args, Debug)]
#[command(long_about = "Disables functionality of the entire pool.

Prerequisites:
- The program's pool state must be initialized prior to the invokation.
")]
pub struct DisablePoolArgs {
    #[arg(
        long,
        short,
        help = "The program's admin or a disable pool authority signer. Defaults to config wallet if not set."
    )]
    pub authority: Option<String>,
}

impl DisablePoolArgs {
    pub async fn run(args: crate::Args) {
        let Self { authority } = match args.subcmd {
            Subcmd::DisablePool(a) => a,
            _ => unreachable!(),
        };

        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let authority_signer = authority.map(|s| parse_signer(&s).unwrap());
        let authority = authority_signer.as_ref().unwrap_or(&payer);

        let pool_state_pda = find_pool_state_address(program_id).0;
        let pool_state_data = rpc.get_account_data(&pool_state_pda).await.unwrap();
        let pool_state = try_pool_state(&pool_state_data).unwrap();

        let disable_pool_authority_list_pda =
            find_disable_pool_authority_list_address(program_id).0;
        let disable_pool_authority_list_data = rpc
            .get_account_data(&disable_pool_authority_list_pda)
            .await
            .unwrap();
        let disable_pool_authority_list =
            try_disable_pool_authority_list(&disable_pool_authority_list_data).unwrap();

        // TODO: maybe if pool is rebalancing, the command should wait keep refetching and checking the pool state
        verify_pool_is_not_rebalancing(pool_state).unwrap();

        // check if authority is either the admin or a disable pool authority
        if pool_state.admin != authority.pubkey() {
            verify_disable_pool_authority(disable_pool_authority_list, authority.pubkey()).unwrap();
        }

        let ix = disable_pool_ix_with_program_id(
            program_id,
            DisablePoolKeys {
                signer: authority.pubkey(),
                pool_state: pool_state_pda,
                disable_pool_authority_list: disable_pool_authority_list_pda,
            },
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), authority.as_ref()];
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
