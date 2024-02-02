use assert_cmd::Command;
use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use sanctum_solana_test_utils::{
    banks_rpc_server::BanksRpcServer,
    cli::TempCliConfig,
    test_fixtures_dir,
    token::{tokenkeg::TokenkegProgramTest, MockMintArgs},
    ExtendedProgramTest,
};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

use crate::common::{base_cmd, SctrProgramTest, TestSctrCmd};

pub async fn setup_with_init_auth_as_payer(
    pt: ProgramTest,
    mock_auth_kp: &Keypair,
) -> (Command, TempCliConfig, BanksClient) {
    let (bc, _rng_payer, _rbh) = pt.start().await;
    let (port, _jh) = BanksRpcServer::spawn_random_unused(bc.clone()).await;
    let cfg = TempCliConfig::from_keypair_and_local_port(mock_auth_kp, port);
    let cmd = base_cmd(&cfg);
    (cmd, cfg, bc)
}

#[tokio::test(flavor = "multi_thread")]
async fn init_success_payer_init_auth() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let lp_mint = Pubkey::new_unique();
    let pt = ProgramTest::default()
        .add_s_program()
        .add_system_account(mock_auth_kp.pubkey(), 1_000_000_000)
        .add_tokenkeg_mint_from_args(
            lp_mint,
            MockMintArgs {
                mint_authority: Some(mock_auth_kp.pubkey()),
                freeze_authority: Some(mock_auth_kp.pubkey()),
                supply: 0,
                decimals: 9,
            },
        );
    let (mut cmd, _cfg, mut bc) = setup_with_init_auth_as_payer(pt, &mock_auth_kp).await;
    cmd.cmd_init().arg(lp_mint.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
}
