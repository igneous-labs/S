use std::process::Output;

use cli_test_utils::TestCliCmd;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;

use crate::common::{setup, GpcSplProgramTest, TestGpcCmd};

#[tokio::test(flavor = "multi_thread")]
async fn init_success() {
    let pt = ProgramTest::default();
    let (mut cmd, _cfg, mut bc, _payer, _rbh) = setup(pt).await;
    cmd.with_spl_calculator().cmd_init();
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    // assert success
    exec_res[0].as_ref().unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn init_alrdy_init() {
    let pt = ProgramTest::default().add_mock_spl_calculator_state(0, Pubkey::default());
    let (mut cmd, _cfg, _bc, _payer, _rbh) = setup(pt).await;
    cmd.with_spl_calculator().cmd_init();
    let Output {
        status,
        stdout,
        stderr,
    } = cmd.output().unwrap();
    assert!(status.success());
    assert!(stdout.is_empty());
    let s = std::str::from_utf8(&stderr).unwrap();
    println!("{s}");
}