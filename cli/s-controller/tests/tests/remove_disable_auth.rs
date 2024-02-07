use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_test_utils::{
    assert_disable_authority_removed, DisablePoolAuthorityListProgramTest, PoolStateProgramTest,
    DEFAULT_POOL_STATE,
};
use solana_program_test::ProgramTest;
use solana_sdk::pubkey::Pubkey;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn remove_disable_auth_success_payer_init_auth() {
    let disable_auth_to_remove = Pubkey::new_unique();
    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE)
        .add_disable_pool_authority_list(&[disable_auth_to_remove]);
    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    cmd.cmd_remove_disable_auth()
        .arg(disable_auth_to_remove.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_disable_authority_removed(&mut bc, disable_auth_to_remove, 1).await;
}
