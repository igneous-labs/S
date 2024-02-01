use assert_cmd::Command;
use cli_test_utils::TestCliCmd;
use sanctum_solana_test_utils::{banks_rpc_server::BanksRpcServer, cli::TempCliConfig};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{hash::Hash, signature::Keypair};

pub fn cargo_bin() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn base_cmd(cfg: &TempCliConfig) -> Command {
    let mut cmd = cargo_bin();
    cmd.with_send_mode_dump_msg().with_cfg_temp_cli(cfg);
    cmd
}

fn add_flat_fee_program(mut pt: ProgramTest) -> ProgramTest {
    pt.add_program(
        "flat_fee",
        flat_fee_lib::program::ID,
        processor!(flat_fee::entrypoint::process_instruction),
    );
    pt
}

pub async fn setup(pt: ProgramTest) -> (Command, TempCliConfig, BanksClient, Keypair, Hash) {
    let pt = add_flat_fee_program(pt);
    let (bc, payer, rbh) = pt.start().await;

    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc, payer, rbh)
}

pub trait TestCmd {
    fn with_flat_fee_program(&mut self) -> &mut Self;

    fn cmd_initialize(&mut self) -> &mut Self;
}

impl TestCmd for Command {
    fn with_flat_fee_program(&mut self) -> &mut Self {
        self.arg(flat_fee_lib::program::ID_STR)
    }

    fn cmd_initialize(&mut self) -> &mut Self {
        self.arg("initialize")
    }
}
