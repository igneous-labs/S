use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use generic_pool_calculator_lib::utils::try_calculator_state;
use sanctum_solana_test_utils::{cli::temp_keypair_file, ExtendedBanksClient};
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_stake_pool_keys::spl_stake_pool_program;
use test_utils::SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT;

use crate::common::{setup, setup_with_payer_as_manager, GpcSplProgramTest, TestGpcCmd};

async fn assert_last_upgrade_slot_updated(bc: &mut BanksClient, expected_slot: u64) {
    let state_data = bc
        .get_account_data(spl_calculator_lib::program::SPL_CALCULATOR_STATE_ID)
        .await;
    let state = try_calculator_state(&state_data).unwrap();
    assert_eq!(state.last_upgrade_slot, expected_slot);
}

#[tokio::test(flavor = "multi_thread")]
async fn update_last_upgrade_slot_success_payer_as_manager_new_manager_pubkey() {
    let (mut cmd, _cfg, mut bc, _payer, _rbh) = setup_with_payer_as_manager(0).await;
    cmd.with_spl_calculator()
        .cmd_update_last_upgrade_slot()
        .arg(spl_stake_pool_program::ID_STR);
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_last_upgrade_slot_updated(&mut bc, SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn set_manager_success_separate_manager_new_manager_pubkey() {
    let curr_manager = Keypair::new();
    let curr_manager_keyfile = temp_keypair_file(&curr_manager);
    let pt = ProgramTest::default().add_mock_spl_calculator_state(0, curr_manager.pubkey());
    let (mut cmd, _cfg, mut bc, _payer, _rbh) = setup(pt).await;
    cmd.with_spl_calculator()
        .cmd_update_last_upgrade_slot()
        .arg("-c")
        .arg(curr_manager_keyfile.path())
        .arg(spl_stake_pool_program::ID_STR);
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_last_upgrade_slot_updated(&mut bc, SPL_STAKE_POOL_PROG_LAST_UPDATED_SLOT).await;
}
