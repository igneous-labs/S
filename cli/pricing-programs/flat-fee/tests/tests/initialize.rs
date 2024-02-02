use cli_test_utils::TestCliCmd;
use solana_program_test::ProgramTest;
use solana_sdk::signature::Keypair;

use crate::common::{setup, TestCmd};

#[tokio::test(flavor = "multi_thread")]
async fn initialize_success() {
    let payer = Keypair::new();

    let (mut cmd, _cfg, mut bc, _payer, _rbh) =
        setup(ProgramTest::default(), payer, None, &[], &[]).await;

    cmd.with_flat_fee_program().cmd_initialize();

    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    // assert success
    exec_res[0].as_ref().unwrap().result.as_ref().unwrap();
}
