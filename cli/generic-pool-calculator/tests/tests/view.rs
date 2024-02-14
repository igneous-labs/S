use std::process::Output;

use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;

use crate::common::{setup, GpcSplProgramTest, TestGpcCmd};

#[tokio::test(flavor = "multi_thread")]
async fn view_success_program_str() {
    let pt = ProgramTest::default().add_mock_spl_calculator_state(0, Pubkey::new_unique());
    let (mut cmd, _cfg, _bc, _payer, _rbh) = setup(pt).await;
    cmd.with_spl_calculator_str().cmd_view();
    let Output { status, stdout, .. } = cmd.output().unwrap();
    assert!(status.success());
    eprintln!("{}", std::str::from_utf8(&stdout).unwrap());
}
