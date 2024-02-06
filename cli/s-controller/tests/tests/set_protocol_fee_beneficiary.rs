use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_lib::try_pool_state;
use s_controller_test_utils::{PoolStateBanksClient, PoolStateProgramTest, DEFAULT_POOL_STATE};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::pubkey::Pubkey;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

async fn assert_protocol_fee_beneficiary(bc: &mut BanksClient, protocol_fee_beneficiary: Pubkey) {
    let pool_state_acc = bc.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert!(pool_state.protocol_fee_beneficiary == protocol_fee_beneficiary);
}

#[tokio::test(flavor = "multi_thread")]
async fn set_admin_success_payer_admin() {
    let new_protocol_fee_beneficiary = Pubkey::new_unique();
    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_set_protocol_fee_beneficiary()
        .arg(new_protocol_fee_beneficiary.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_protocol_fee_beneficiary(&mut bc, new_protocol_fee_beneficiary).await;
}
