//! TODO: need to implement getProgramAccounts on BanksRpcServer for this command's tests to work

/*
use std::process::Output;

use flat_fee_interface::ProgramState;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::signature::Keypair;

use crate::common::{setup, TestCmd};

#[tokio::test(flavor = "multi_thread")]
async fn view_success() {
    let program_state = ProgramState {
        manager: Pubkey::default(),
        lp_withdrawal_fee_bps: Default::default(),
    };

    let (mut cmd, _cfg, _bc, _payer, _rbh) = setup(
        ProgramTest::default(),
        Keypair::new(),
        Some(program_state),
        &[],
        &[],
    )
    .await;

    cmd.with_flat_fee_program().cmd_view();

    // TODO: need to implement getProgramAccounts on BanksRpcServer
    // for this test to work
    let Output { status, stdout, .. } = cmd.output().unwrap();
    assert!(status.success());
    eprintln!("{}", std::str::from_utf8(&stdout).unwrap());
}
 */
