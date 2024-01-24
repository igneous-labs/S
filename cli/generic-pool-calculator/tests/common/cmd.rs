use assert_cmd::Command;
use cli_test_utils::TestCliCmd;
use sanctum_solana_test_utils::{
    banks_rpc_server::BanksRpcServer, cli::TempCliConfig, ExtendedProgramTest,
};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{hash::Hash, signature::Keypair};

pub fn cargo_bin() -> Command {
    Command::cargo_bin("gpc").unwrap()
}

pub async fn setup(pt: ProgramTest) -> (Command, TempCliConfig, BanksClient, Keypair, Hash) {
    // test against spl calculator
    let mut pt = pt
        .add_test_fixtures_account("spl-stake-pool-prog.json")
        .add_test_fixtures_account("spl-stake-pool-prog-data.json");
    pt.add_program(
        "spl_calculator",
        spl_calculator_lib::program::ID,
        processor!(spl_calculator::entrypoint::process_instruction),
    );

    let (bc, payer, rbh) = pt.start().await;

    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
    let mut cmd = cargo_bin();
    cmd.with_b64_send_mode().with_temp_cli_cfg(&cfg);
    (cmd, cfg, bc, payer, rbh)
}

pub trait TestGpcCmd {
    fn with_spl_calculator(&mut self) -> &mut Self;

    fn cmd_init(&mut self) -> &mut Self;
}

impl TestGpcCmd for Command {
    fn with_spl_calculator(&mut self) -> &mut Self {
        self.arg(spl_calculator_lib::program::ID_STR)
    }

    fn cmd_init(&mut self) -> &mut Self {
        self.arg("init")
    }
}
