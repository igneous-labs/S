use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_test_utils::{
    assert_lst_input_disabled, assert_lst_input_enabled, jito_marinade_no_fee_program_test,
    JitoMarinadeProgramTestArgs,
};
use solana_sdk::pubkey::Pubkey;
use test_utils::jitosol;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn disable_jito_input_success_payer_init_auth() {
    let pt = jito_marinade_no_fee_program_test(JitoMarinadeProgramTestArgs {
        // all don't cares
        jitosol_sol_value: 0,
        msol_sol_value: 0,
        jitosol_reserves: 0,
        msol_reserves: 0,
        jitosol_protocol_fee_accumulator: 0,
        msol_protocol_fee_accumulator: 0,
        lp_token_mint: Pubkey::new_unique(),
        lp_token_supply: 0,
    })
    .add_s_program();
    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    assert_lst_input_enabled(&mut bc, jitosol::ID).await;
    cmd.cmd_disable_lst_input().arg(jitosol::ID_STR);
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_lst_input_disabled(&mut bc, jitosol::ID).await;
}
