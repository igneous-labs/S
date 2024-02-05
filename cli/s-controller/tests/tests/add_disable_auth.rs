use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_lib::try_disable_pool_authority_list;
use s_controller_test_utils::{
    DisablePoolAuthorityListBanksClient, PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::pubkey::Pubkey;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

async fn assert_new_disable_auth_added(bc: &mut BanksClient, new_disable_auth: Pubkey) {
    let disable_list_acc = bc.get_disable_pool_list_acc().await;
    let disable_list = try_disable_pool_authority_list(&disable_list_acc.data).unwrap();
    assert!(disable_list.contains(&new_disable_auth));
}

#[tokio::test(flavor = "multi_thread")]
async fn add_disable_auth_success_payer_init_auth() {
    let new_disable_auth = Pubkey::new_unique();
    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);
    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    cmd.cmd_add_disable_auth().arg(new_disable_auth.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_new_disable_auth_added(&mut bc, new_disable_auth).await;
}
