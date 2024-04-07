use s_controller_test_utils::{
    assert_pricing_prog_set, jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs,
};
use sanctum_solana_test_utils::cli::{assert_all_txs_success_nonempty, ExtendedCommand};
use solana_program_test::ProgramTest;
use solana_sdk::pubkey::Pubkey;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

fn jito_marinade_no_fee_pt_with_flat_fee_prog() -> ProgramTest {
    jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        // all dont cares
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_s_program()
    .add_flat_fee_pricing_program()
}

#[tokio::test(flavor = "multi_thread")]
async fn set_pricing_prog_flat_fee_success_payer_admin() {
    let pt = jito_marinade_no_fee_pt_with_flat_fee_prog();

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_set_pricing_prog().arg("flat-fee");
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_pricing_prog_set(&mut bc, flat_fee_lib::program::ID).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn set_pricing_prog_flat_fee_pubkey_arg_success_payer_admin() {
    let pt = jito_marinade_no_fee_pt_with_flat_fee_prog();

    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;

    cmd.cmd_set_pricing_prog()
        .arg(flat_fee_lib::program::ID_STR);
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_pricing_prog_set(&mut bc, flat_fee_lib::program::ID).await;
}
