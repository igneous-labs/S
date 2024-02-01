use assert_cmd::Command;
use cli_test_utils::TestCliCmd;
use flat_fee_lib::utils::try_program_state_mut;
use sanctum_solana_test_utils::{
    banks_rpc_server::BanksRpcServer, cli::TempCliConfig, ExtendedProgramTest,
};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{account::Account, hash::Hash, signature::Keypair, signer::Signer};

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
    let (bc, payer, rbh) = add_flat_fee_program(pt).start().await;

    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc, payer, rbh)
}

pub async fn setup_with_payer_as_manager(
    pt: ProgramTest,
) -> (Command, TempCliConfig, BanksClient, Keypair, Hash) {
    let payer = Keypair::new();

    let mut data = vec![0u8; flat_fee_lib::program::STATE_SIZE];
    let state = try_program_state_mut(&mut data).unwrap();
    state.manager = payer.pubkey();
    let state_acc = Account {
        lamports: 1_000_000_000, // just do 1 sol who gives a f
        data,
        owner: flat_fee_lib::program::ID,
        executable: false,
        rent_epoch: u64::MAX,
    };

    let (bc, _rng_payer, rbh) = add_flat_fee_program(pt)
        .add_system_account(payer.pubkey(), 1_000_000_000)
        .add_account_chained(flat_fee_lib::program::STATE_ID, state_acc)
        .start()
        .await;

    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc, payer, rbh)
}

pub trait TestCmd {
    fn with_flat_fee_program(&mut self) -> &mut Self;

    fn cmd_initialize(&mut self) -> &mut Self;

    fn cmd_set_manager(&mut self) -> &mut Self;
}

impl TestCmd for Command {
    fn with_flat_fee_program(&mut self) -> &mut Self {
        self.arg(flat_fee_lib::program::ID_STR)
    }

    fn cmd_initialize(&mut self) -> &mut Self {
        self.arg("initialize")
    }

    fn cmd_set_manager(&mut self) -> &mut Self {
        self.arg("set-manager")
    }
}
