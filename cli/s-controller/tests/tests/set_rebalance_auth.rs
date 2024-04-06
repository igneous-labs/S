use s_controller_test_utils::{
    assert_rebalance_authority, PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::cli::{assert_all_txs_success_nonempty, ExtendedCommand};
use solana_program_test::ProgramTest;
use solana_sdk::pubkey::Pubkey;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn set_rebalance_auth_success_payer_admin() {
    let new_rebalance_auth = Pubkey::new_unique();

    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_set_rebalance_auth()
        .arg(new_rebalance_auth.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_rebalance_authority(&mut bc, new_rebalance_auth).await;
}
