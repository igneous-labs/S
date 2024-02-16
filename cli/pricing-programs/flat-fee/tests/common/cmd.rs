use assert_cmd::Command;
use cli_test_utils::TestCliCmd;
use flat_fee_interface::ProgramState;
use flat_fee_test_utils::{FlatFeeProgramTest, MockFeeAccount, MockFeeAccountArgs};
use sanctum_solana_test_utils::{
    banks_rpc_server::BanksRpcServer,
    cli::TempCliConfig,
    token::{tokenkeg::TokenkegProgramTest, MockMintArgs},
    ExtendedProgramTest, IntoAccount,
};
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{hash::Hash, signature::Keypair, signer::Signer};

fn add_flat_fee_program(mut pt: ProgramTest) -> ProgramTest {
    pt.add_program(
        "flat_fee",
        flat_fee_lib::program::ID,
        processor!(flat_fee::entrypoint::process_instruction),
    );
    pt
}

// pub async fn setup(pt: ProgramTest) -> (Command, TempCliConfig, BanksClient, Keypair, Hash) {
//     let (bc, payer, rbh) = add_flat_fee_program(pt).start().await;

//     let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
//     let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
//     let cmd = base_cmd(&cfg);
//     (cmd, cfg, bc, payer, rbh)
// }

// setup program test
// - `program_state`      sets flat fee program state up
// - `mock_mints`         sets token mint up
// - `mock_fee_accounts`  sets token mint up and set mock fee account
// - funds given payer and set it as default cli keypair in temporary config
pub async fn setup(
    pt: ProgramTest,
    payer: Keypair,
    program_state: Option<ProgramState>,
    mock_mints: &[Pubkey],
    mock_fee_accounts: &[MockFeeAccountArgs],
) -> (Command, TempCliConfig, BanksClient, Keypair, Hash) {
    let mut pt = add_flat_fee_program(pt).add_system_account(payer.pubkey(), 1_000_000_000);

    if let Some(program_state) = program_state {
        pt.add_mock_program_state(program_state);
    }

    for &mint in mock_mints {
        pt = pt.add_tokenkeg_mint_from_args(
            mint,
            MockMintArgs {
                mint_authority: None,
                freeze_authority: None,
                supply: 0,
                decimals: 9,
            },
        );
    }

    for mfa in mock_fee_accounts {
        pt = pt.add_tokenkeg_mint_from_args(
            mfa.lst_mint,
            MockMintArgs {
                mint_authority: None,
                freeze_authority: None,
                supply: 0,
                decimals: 9,
            },
        );
        let (acc, addr) = mfa.to_fee_account_and_addr(flat_fee_lib::program::ID);
        pt.add_account(addr, MockFeeAccount(acc).into_account())
    }

    let (bc, _rng_payer, rbh) = pt.start().await;

    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(&payer, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc, payer, rbh)
}

pub fn cargo_bin() -> Command {
    Command::cargo_bin("flt").unwrap()
}

fn base_cmd(cfg: &TempCliConfig) -> Command {
    let mut cmd = cargo_bin();
    cmd.with_send_mode_dump_msg().with_cfg_temp_cli(cfg);
    cmd
}

pub trait TestCmd {
    fn with_flat_fee_program(&mut self) -> &mut Self;

    fn cmd_initialize(&mut self) -> &mut Self;

    fn cmd_set_manager(&mut self) -> &mut Self;

    fn cmd_set_lp_withdrawal_fee(&mut self) -> &mut Self;

    fn cmd_add_lst(&mut self) -> &mut Self;

    fn cmd_remove_lst(&mut self) -> &mut Self;

    fn cmd_set_lst_fee(&mut self) -> &mut Self;

    fn cmd_view(&mut self) -> &mut Self;

    fn cmd_view_lst(&mut self) -> &mut Self;
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

    fn cmd_set_lp_withdrawal_fee(&mut self) -> &mut Self {
        self.arg("set-lp-withdrawal-fee")
    }

    fn cmd_add_lst(&mut self) -> &mut Self {
        self.arg("add-lst")
    }

    fn cmd_remove_lst(&mut self) -> &mut Self {
        self.arg("remove-lst")
    }

    fn cmd_set_lst_fee(&mut self) -> &mut Self {
        self.arg("set-lst-fee")
    }

    fn cmd_view(&mut self) -> &mut Self {
        self.arg("view")
    }

    fn cmd_view_lst(&mut self) -> &mut Self {
        self.arg("view-lst")
    }
}
