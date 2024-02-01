// use std::process::Output;

use cli_test_utils::TestCliCmd;
use flat_fee_lib::utils::try_program_state;
use sanctum_solana_test_utils::ExtendedBanksClient;
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClient, ProgramTest};

use crate::common::{setup_with_payer_as_manager, TestCmd};

async fn assert_new_manager(bc: &mut BanksClient, expected_new_manager: Pubkey) {
    let state_data = bc.get_account_data(flat_fee_lib::program::STATE_ID).await;
    let state = try_program_state(&state_data).unwrap();
    assert_eq!(state.manager, expected_new_manager);
}

#[tokio::test(flavor = "multi_thread")]
async fn set_manager_success_payer_as_manager_new_manager_pubkey() {
    let pt = ProgramTest::default();
    let new_manager = Pubkey::new_unique();
    let (mut cmd, _cfg, mut bc, _payer, _rbh) = setup_with_payer_as_manager(pt).await;
    cmd.with_flat_fee_program()
        .cmd_set_manager()
        .arg(new_manager.to_string());
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    exec_res[0].as_ref().unwrap();
    assert_new_manager(&mut bc, new_manager).await;
}
