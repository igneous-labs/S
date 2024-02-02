use cli_test_utils::{assert_all_txs_success, TestCliCmd};
use generic_pool_calculator_lib::utils::try_calculator_state;
use sanctum_solana_test_utils::{cli::temp_keypair_file, ExtendedBanksClient};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::common::{setup, setup_with_payer_as_manager, GpcSplProgramTest, TestGpcCmd};

async fn assert_new_manager(bc: &mut BanksClient, expected_new_manager: Pubkey) {
    let state_data = bc
        .get_account_data(spl_calculator_lib::program::SPL_CALCULATOR_STATE_ID)
        .await;
    let state = try_calculator_state(&state_data).unwrap();
    assert_eq!(state.manager, expected_new_manager);
}

#[tokio::test(flavor = "multi_thread")]
async fn set_manager_success_payer_as_manager_new_manager_pubkey() {
    let new_manager = Pubkey::new_unique();
    let (mut cmd, _cfg, mut bc, _payer, _rbh) = setup_with_payer_as_manager(0).await;
    cmd.with_spl_calculator()
        .cmd_set_manager()
        .arg(new_manager.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success(&exec_res);
    assert_new_manager(&mut bc, new_manager).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn set_manager_success_separate_manager_new_manager_pubkey() {
    let new_manager = Pubkey::new_unique();
    let curr_manager = Keypair::new();
    let curr_manager_keyfile = temp_keypair_file(&curr_manager);
    let pt = ProgramTest::default().add_mock_spl_calculator_state(0, curr_manager.pubkey());
    let (mut cmd, _cfg, mut bc, _payer, _rbh) = setup(pt).await;
    cmd.with_spl_calculator()
        .cmd_set_manager()
        .arg("-c")
        .arg(curr_manager_keyfile.path())
        .arg(new_manager.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success(&exec_res);
    assert_new_manager(&mut bc, new_manager).await;
}
