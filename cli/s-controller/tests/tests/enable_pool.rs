use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_interface::PoolState;
use s_controller_test_utils::{
    assert_pool_disabled, assert_pool_enabled, PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use solana_program_test::ProgramTest;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn enable_pool_success_payer_admin() {
    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(PoolState {
            is_disabled: 1,
            ..DEFAULT_POOL_STATE
        });

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    assert_pool_disabled(&mut bc).await;

    cmd.cmd_enable_pool();
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_pool_enabled(&mut bc).await;
}
