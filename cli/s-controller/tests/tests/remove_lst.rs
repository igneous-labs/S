use s_controller_lib::FindLstPdaAtaKeys;
use s_controller_test_utils::{
    assert_lst_removed, AddSplProgramTest, LstStateListProgramTest, MockLstStateArgs,
    PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::cli::{assert_all_txs_success_nonempty, ExtendedCommand};
use solana_program_test::ProgramTest;
use test_utils::jitosol;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn remove_lst_jito_success_payer_init_auth() {
    let pt = ProgramTest::default()
        .add_spl_progs()
        .add_jito_stake_pool()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE)
        .add_mock_lst_states(&[MockLstStateArgs {
            mint: jitosol::ID,
            sol_value_calculator: spl_calculator_lib::program::ID,
            token_program: spl_token::ID,
            sol_value: 0,
            reserves_amt: 0,
            protocol_fee_accumulator_amt: 0,
            is_input_disabled: true,
        }]);
    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    cmd.cmd_remove_lst().arg(jitosol::ID_STR);
    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    assert_lst_removed(
        &mut bc,
        FindLstPdaAtaKeys {
            lst_mint: jitosol::ID,
            token_program: spl_token::ID,
        },
        1,
    )
    .await;
}
