use assert_cmd::Command;
use cli_test_utils::TestCliCmd;
use sanctum_solana_test_utils::{
    banks_rpc_server::BanksRpcServer, cli::TempCliConfig, ExtendedProgramTest,
};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::{hash::Hash, signature::Keypair, signer::Signer};

use super::GpcSplProgramTest;

pub fn cargo_bin() -> Command {
    Command::cargo_bin("gpc").unwrap()
}

fn base_cmd(cfg: &TempCliConfig) -> Command {
    let mut cmd = cargo_bin();
    cmd.with_send_mode_dump_msg().with_cfg_temp_cli(cfg);
    cmd
}

pub async fn setup_with_payer_as_manager(
    last_upgrade_slot: u64,
) -> (Command, TempCliConfig, BanksClient, Keypair, Hash) {
    let payer = Keypair::new();
    let pt = ProgramTest::default()
        .add_spl_programs()
        .add_system_account(payer.pubkey(), 1_000_000_000)
        .add_mock_spl_calculator_state(last_upgrade_slot, payer.pubkey());
    let (bc, _rng_payer, rbh) = pt.start().await;

    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc, payer, rbh)
}

pub async fn setup(pt: ProgramTest) -> (Command, TempCliConfig, BanksClient, Keypair, Hash) {
    let pt = pt.add_spl_programs();
    let (bc, payer, rbh) = pt.start().await;

    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc, payer, rbh)
}

pub trait TestGpcCmd {
    fn with_spl_calculator(&mut self) -> &mut Self;

    fn with_spl_calculator_str(&mut self) -> &mut Self;

    fn cmd_init(&mut self) -> &mut Self;

    fn cmd_set_manager(&mut self) -> &mut Self;

    fn cmd_update_last_upgrade_slot(&mut self) -> &mut Self;

    fn cmd_view(&mut self) -> &mut Self;
}

impl TestGpcCmd for Command {
    fn with_spl_calculator(&mut self) -> &mut Self {
        self.arg(spl_calculator_lib::program::ID_STR)
    }

    fn with_spl_calculator_str(&mut self) -> &mut Self {
        self.arg("spl")
    }

    fn cmd_init(&mut self) -> &mut Self {
        self.arg("init")
    }

    fn cmd_set_manager(&mut self) -> &mut Self {
        self.arg("set-manager")
    }

    fn cmd_update_last_upgrade_slot(&mut self) -> &mut Self {
        self.arg("update-last-upgrade-slot")
    }

    fn cmd_view(&mut self) -> &mut Self {
        self.arg("view")
    }
}
