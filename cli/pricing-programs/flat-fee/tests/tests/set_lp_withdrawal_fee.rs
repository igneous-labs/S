use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use flat_fee_interface::ProgramState;
use flat_fee_lib::utils::try_program_state;
use sanctum_solana_test_utils::ExtendedBanksClient;
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::common::{setup, TestCmd};

async fn assert_lp_withdrawal_fee_bps(bc: &mut BanksClient, expected_lp_withdrawal_fee_bps: u16) {
    let state_data = bc.get_account_data(flat_fee_lib::program::STATE_ID).await;
    let state = try_program_state(&state_data).unwrap();
    assert_eq!(state.lp_withdrawal_fee_bps, expected_lp_withdrawal_fee_bps);
}

#[tokio::test(flavor = "multi_thread")]
async fn set_lp_withdrawal_fee_success() {
    const NEW_LP_WITHDRAWAL_FEE_BPS: u16 = 420;

    let payer = Keypair::new();

    let program_state = ProgramState {
        manager: payer.pubkey(),
        lp_withdrawal_fee_bps: Default::default(),
    };
    let pt = ProgramTest::default();

    let (mut cmd, _cfg, mut bc, _payer, _rbh) =
        setup(pt, payer, Some(program_state), &[], &[]).await;

    cmd.with_flat_fee_program()
        .cmd_set_lp_withdrawal_fee()
        .arg(NEW_LP_WITHDRAWAL_FEE_BPS.to_string());

    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_lp_withdrawal_fee_bps(&mut bc, NEW_LP_WITHDRAWAL_FEE_BPS).await;
}
