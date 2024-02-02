// use std::process::Output;

use cli_test_utils::TestCliCmd;
use solana_program_test::ProgramTest;

use crate::common::{setup, TestCmd};

#[tokio::test(flavor = "multi_thread")]
async fn initialize_success() {
    let pt = ProgramTest::default();
    let (mut cmd, _cfg, mut bc, _payer, _rbh) = setup(pt).await;
    cmd.with_flat_fee_program().cmd_initialize();
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    // assert success
    exec_res[0].as_ref().unwrap().result.as_ref().unwrap();
}
