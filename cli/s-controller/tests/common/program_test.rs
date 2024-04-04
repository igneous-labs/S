use assert_cmd::Command;
use sanctum_solana_test_utils::{
    banks_rpc_server::BanksRpcServer, cli::TempCliConfig, test_fixtures_dir, ExtendedProgramTest,
};
use solana_program_test::{processor, BanksClient, ProgramTest, ProgramTestContext};
use solana_sdk::{
    clock::Clock,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};
use test_utils::JITO_STAKE_POOL_LAST_UPDATE_EPOCH;

use super::base_cmd;

pub trait SctrProgramTest {
    fn add_s_program(self) -> Self;

    fn add_flat_fee_pricing_program(self) -> Self;

    fn add_stakedex_program(self) -> Self;
}

impl SctrProgramTest for ProgramTest {
    fn add_s_program(mut self) -> Self {
        self.add_program(
            "s_controller",
            s_controller_lib::program::ID,
            processor!(s_controller::entrypoint::process_instruction),
        );
        self
    }

    fn add_flat_fee_pricing_program(mut self) -> Self {
        self.add_program(
            "flat_fee",
            flat_fee_lib::program::ID,
            processor!(flat_fee::entrypoint::process_instruction),
        );
        self
    }

    fn add_stakedex_program(self) -> Self {
        self.add_test_fixtures_account("stakedex-prog.json")
            .add_test_fixtures_account("stakedex-prog-data.json")
    }
}

pub async fn setup_with_init_auth_as_payer(
    pt: ProgramTest,
) -> (Command, TempCliConfig, BanksClient, Keypair) {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    setup_with_payer(pt, mock_auth_kp).await
}

pub async fn setup_with_payer(
    pt: ProgramTest,
    payer: Keypair,
) -> (Command, TempCliConfig, BanksClient, Keypair) {
    let pt = pt.add_system_account(payer.pubkey(), 1_000_000_000);
    let ctx = pt.start_with_context().await;
    ctx.set_sysvar(&Clock {
        epoch: JITO_STAKE_POOL_LAST_UPDATE_EPOCH,
        ..Default::default()
    });
    let ProgramTestContext { banks_client, .. } = ctx;
    let (port, _jh) = BanksRpcServer::spawn_random_unused(banks_client.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, banks_client, payer)
}
