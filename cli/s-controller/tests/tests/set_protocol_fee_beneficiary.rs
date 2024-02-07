use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_interface::PoolState;
use s_controller_lib::try_pool_state;
use s_controller_test_utils::{PoolStateBanksClient, PoolStateProgramTest, DEFAULT_POOL_STATE};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::common::{setup_with_payer, SctrProgramTest, TestSctrCmd};

async fn assert_protocol_fee_beneficiary(bc: &mut BanksClient, protocol_fee_beneficiary: Pubkey) {
    let pool_state_acc = bc.get_pool_state_acc().await;
    let pool_state = try_pool_state(&pool_state_acc.data).unwrap();
    assert!(pool_state.protocol_fee_beneficiary == protocol_fee_beneficiary);
}

#[tokio::test(flavor = "multi_thread")]
async fn set_protocol_fee_beneficiary_success_payer_beneficiary() {
    let new_protocol_fee_beneficiary = Pubkey::new_unique();
    let curr_protocol_fee_beneficiary = Keypair::new();
    let pt = ProgramTest::default()
        .add_s_program()
        .add_pool_state(PoolState {
            protocol_fee_beneficiary: curr_protocol_fee_beneficiary.pubkey(),
            ..DEFAULT_POOL_STATE
        });

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) =
        setup_with_payer(pt, curr_protocol_fee_beneficiary).await;

    cmd.cmd_set_protocol_fee_beneficiary()
        .arg(new_protocol_fee_beneficiary.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_protocol_fee_beneficiary(&mut bc, new_protocol_fee_beneficiary).await;
}
