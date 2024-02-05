use assert_cmd::Command;
use sanctum_solana_test_utils::{
    banks_rpc_server::BanksRpcServer, cli::TempCliConfig, test_fixtures_dir, ExtendedProgramTest,
};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

use super::base_cmd;

pub trait SctrProgramTest {
    fn add_s_program(self) -> Self;
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
}

pub async fn setup_with_init_auth_as_payer(
    pt: ProgramTest,
) -> (Command, TempCliConfig, BanksClient, Keypair) {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let pt = pt.add_system_account(mock_auth_kp.pubkey(), 1_000_000_000);
    let (bc, _rng_payer, _rbh) = pt.start().await;
    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&mock_auth_kp, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc, mock_auth_kp)
}
