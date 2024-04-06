use s_controller_lib::try_pool_state;
use s_controller_test_utils::{PoolStateBanksClient, PoolStateProgramTest, DEFAULT_POOL_STATE};
use sanctum_solana_test_utils::cli::{assert_all_txs_success_nonempty, ExtendedCommand};
use solana_program_test::{BanksClient, ProgramTest};

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

async fn assert_protocol_fee(
    bc: &mut BanksClient,
    trading_protocol_fee_bps: u16,
    lp_protocol_fee_bps: u16,
) {
    let pool_state_acc = bc.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert!(pool_state.trading_protocol_fee_bps == trading_protocol_fee_bps);
    assert!(pool_state.lp_protocol_fee_bps == lp_protocol_fee_bps);
}

#[tokio::test(flavor = "multi_thread")]
async fn set_protocol_fee_success_both_fees_payer_init_auth() {
    const NEW_TRADING_PROTOCOL_FEE_BPS: u16 = 420;
    const NEW_LP_PROTOCOL_FEE_BPS: u16 = 69;

    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_set_protocol_fee()
        .arg("--trading-fee")
        .arg(NEW_TRADING_PROTOCOL_FEE_BPS.to_string())
        .arg("--lp-fee")
        .arg(NEW_LP_PROTOCOL_FEE_BPS.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_protocol_fee(
        &mut bc,
        NEW_TRADING_PROTOCOL_FEE_BPS,
        NEW_LP_PROTOCOL_FEE_BPS,
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn set_protocol_fee_success_trading_fee_payer_init_auth() {
    const NEW_TRADING_PROTOCOL_FEE_BPS: u16 = 420;

    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_set_protocol_fee()
        .arg("--trading-fee")
        .arg(NEW_TRADING_PROTOCOL_FEE_BPS.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_protocol_fee(&mut bc, NEW_TRADING_PROTOCOL_FEE_BPS, Default::default()).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn set_protocol_fee_success_lp_fee_payer_init_auth() {
    const NEW_LP_PROTOCOL_FEE_BPS: u16 = 69;

    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_set_protocol_fee()
        .arg("--lp-fee")
        .arg(NEW_LP_PROTOCOL_FEE_BPS.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_protocol_fee(&mut bc, Default::default(), NEW_LP_PROTOCOL_FEE_BPS).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn set_protocol_fee_failure_no_fee_payer_init_auth() {
    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);

    let (mut cmd, _cfg, _bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_set_protocol_fee().assert().failure();
}
