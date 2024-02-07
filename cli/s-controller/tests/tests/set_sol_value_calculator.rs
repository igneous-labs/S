use cli_test_utils::{assert_all_txs_success_nonempty, TestCliCmd};
use s_controller_test_utils::{
    AddSplProgramTest, LstStateListProgramTest, MockLstStateArgs, PoolStateProgramTest,
    DEFAULT_POOL_STATE,
};
use solana_program_test::ProgramTest;
use solana_sdk::system_program;
use test_utils::jitosol;

use crate::common::{setup_with_init_auth_as_payer, SctrProgramTest, TestSctrCmd};

#[tokio::test(flavor = "multi_thread")]
async fn set_sol_value_calculator_jito_success_payer_init_auth() {
    const SPL_CALC_JITO_ACC_SUFFIXES_PUBKEY_STR: [&str; 5] = [
        "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn",
        "7orJ4kDhn1Ewp54j29tBzUWDFGhyimhYi7sxybZcphHd",
        "Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb",
        "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy",
        "EmiU8AQkB2sswTxVB6aCmsAJftoowZGGDXuytm6X65R3",
    ];

    let pt = ProgramTest::default()
        .add_spl_progs()
        .add_jito_stake_pool()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE)
        .add_mock_lst_states(&[MockLstStateArgs {
            mint: jitosol::ID,
            sol_value_calculator: system_program::ID,
            token_program: spl_token::ID,
            sol_value: 0,
            reserves_amt: 0,
            protocol_fee_accumulator_amt: 0,
            is_input_disabled: true,
        }]);
    let (mut cmd, _cfg, mut bc, _mock_auth_kp) = setup_with_init_auth_as_payer(pt).await;
    cmd.cmd_set_sol_value_calculator_prog()
        .arg("--sol-val-calc")
        .arg(spl_calculator_lib::program::ID_STR)
        .arg("--mint")
        .arg(jitosol::ID_STR);

    cmd.arg("--account-suffixes");
    for acc_suffix_pubkey_str in SPL_CALC_JITO_ACC_SUFFIXES_PUBKEY_STR {
        cmd.arg(acc_suffix_pubkey_str);
    }
    // TODO: DELETEME
    cmd.unwrap();

    let exec_res = cmd.exec_b64_txs(&mut bc).await;
    assert_all_txs_success_nonempty(&exec_res);
    // TODO: assert changes
}
