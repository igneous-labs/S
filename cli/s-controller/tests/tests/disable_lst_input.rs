use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_lib::{try_lst_state_list, U8Bool};
use s_controller_test_utils::{
    jito_marinade_no_fee_program_test, JitoMarinadeProgramTestArgs, LstStateListBanksClient,
};
use solana_program_test::BanksClient;
use solana_readonly_account::ReadonlyAccountData;
use solana_sdk::pubkey::Pubkey;
use test_utils::jitosol;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

async fn assert_lst_input_disabled(bc: &mut BanksClient, lst_mint: Pubkey) {
    let lst_state_list_acc = bc.get_lst_state_list_acc().await;
    let lst_state_list = try_lst_state_list(&lst_state_list_acc.data()).unwrap();
    let lst_state = lst_state_list.iter().find(|x| x.mint == lst_mint).unwrap();
    assert!(U8Bool(lst_state.is_input_disabled).is_true());
}

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
    cmd.cmd_disable_lst_input().arg(jitosol::ID_STR);
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_lst_input_disabled(&mut bc, jitosol::ID).await;
}
